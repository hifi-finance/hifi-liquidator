//! Liquidations Module
//!
//! This module is responsible for triggering liquidations.
use crate::{vaults::Vault, EthersResult};

use ethers::{
    core::abi::{self, Tokenize},
    middleware::gas_escalator::{GasEscalator, GeometricGasPrice},
    prelude::*,
};
use hifi_liquidator_bindings::UniswapV2Pair;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tracing::{debug, error, info, trace};

/// TxRequest / Hash/ Submitted at time.
type PendingTransactionTuple = (TransactionRequest, TxHash, Instant);

pub struct Liquidator<M> {
    gas_escalator: GeometricGasPrice,
    hifi_flash_swap_address: Address,
    liquidator_address: Address,
    min_profit: U256,
    pub pending_tx_tuples: HashMap<Address, HashMap<Address, PendingTransactionTuple>>,
    uniswap_v2_pair: UniswapV2Pair<M>,
}

/// Public methods for the Liquidator struct.
impl<M: Middleware> Liquidator<M> {
    /// Constructor
    pub fn new(
        client: Arc<M>,
        gas_escalator: GeometricGasPrice,
        hifi_flash_swap_address: Address,
        liquidator_address: Address,
        min_profit: U256,
        uniswap_v2_pair: Address,
    ) -> Self {
        Self {
            gas_escalator,
            hifi_flash_swap_address,
            liquidator_address,
            min_profit,
            pending_tx_tuples: HashMap::new(),
            uniswap_v2_pair: UniswapV2Pair::new(uniswap_v2_pair, client),
        }
    }

    /// Checks if any transactions that have been submitted are now mined and removes them if they are.
    /// Otherwise, it checks that the vaults are still underwater, and if yes it resubmits the liquidation
    /// txs with a higher gas price.
    pub async fn remove_or_replace_tx(
        &mut self,
        now: Instant,
        fy_token: &Address,
        borrower: &Address,
        pending_tx_tuple: &PendingTransactionTuple,
    ) -> EthersResult<(), M> {
        let client = self.uniswap_v2_pair.client();
        let receipt = client
            .get_transaction_receipt(pending_tx_tuple.1)
            .await
            .map_err(ContractError::MiddlewareError)?;

        if let Some(receipt) = receipt {
            self.remove_pending_tx_tuple(fy_token, borrower);

            let status = if receipt.status == Some(1.into()) {
                "Success"
            } else {
                "Fail"
            };

            trace!(
                status = status,
                tx_hash = ?pending_tx_tuple.1,
                gas_used = %receipt.gas_used.unwrap_or_default(),
                fy_token = %fy_token,
                borrower = %borrower,
                "Confirmed"
            );
        } else {
            // It is possible that immediately after replacing a tx, an older tx gets mined. To prevent an infinite loop,
            // we have to check that the tx nonce is still lower than the transaction count of the liquidator's account.
            let client = self.uniswap_v2_pair.client();
            let current_nonce = client
                .get_transaction_count(self.liquidator_address, Some(BlockNumber::Pending))
                .await
                .map_err(ContractError::MiddlewareError)?;
            let pending_tx_nonce = pending_tx_tuple.0.nonce.expect("Nonce must exist on pending tx");

            if current_nonce > pending_tx_nonce {
                debug!(stale_tx_hash = %pending_tx_tuple.1, current_nonce = %current_nonce, stale_tx_nonce = %pending_tx_nonce, "Dropped stale tx");
                return Ok(());
            }

            // Calculate the new gas price based on how much time passed since the tx was broadcast.
            let old_gas_price: U256 = pending_tx_tuple.0.gas_price.expect("Gas price must be set.");
            let new_gas_price = self
                .gas_escalator
                .get_gas_price(old_gas_price, now.duration_since(pending_tx_tuple.2).as_secs());

            // Stop if the new gas price is not higher than the previous gas price.
            if new_gas_price <= old_gas_price {
                return Ok(());
            }

            // Get a mutable reference of the pending tx tuple.
            let replacement_tx_tuple = self
                .pending_tx_tuples
                .get_mut(fy_token)
                .expect("Inner hash map must exist since we're iterating over the map")
                .get_mut(borrower)
                .expect("Pending tx must exist since we're iterating over the map");

            // Bump the gas price.
            replacement_tx_tuple.0.gas_price = Some(new_gas_price);

            // Rebroadcast (TODO: Can we avoid cloning?).
            match client
                .send_transaction(replacement_tx_tuple.0.clone(), None)
                .await
                .map_err(ContractError::MiddlewareError)
            {
                Ok(replacement_tx) => {
                    let replacement_tx_hash = *replacement_tx;
                    replacement_tx_tuple.1 = replacement_tx_hash;
                    replacement_tx_tuple.2 = now;
                    trace!(
                        old_tx_hash = ?pending_tx_tuple.1,
                        new_tx_hash = ?replacement_tx_tuple.1,
                        new_gas_price = %new_gas_price,
                        "Replaced"
                    );
                }
                Err(err) => {
                    self.handle_reverted_tx(err, "Replacement tx reverted with error");
                }
            }
        }

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the BalanceSheet.
    /// It does this with capital sourced from Uniswap V2.
    pub async fn trigger(
        &mut self,
        gas_price: U256,
        vaults: impl Iterator<Item = (&Address, &Address, &Vault)>,
    ) -> EthersResult<(), M> {
        debug!("Checking for under-collateralized positions...");

        let now = Instant::now();

        for (fy_token, borrower, vault) in vaults {
            // Only iterate over tuples that do not have pending liquidations.
            if let Some(pending_tx_tuple) = self.get_pending_tx_tuple(fy_token, borrower) {
                trace!(
                    pending_tx = %pending_tx_tuple.1,
                    fy_token = %fy_token,
                    borrower = %borrower,
                    "Liquidation tx not confirmed yet"
                );
                continue;
            }

            // Skip vaults that are not underwater.
            if !vault.is_account_underwater || vault.debt_in_underlying.is_zero() {
                continue;
            }

            // TODO: log the current collateralization ratio.
            info!(
                fy_token = %fy_token,
                borrower = %borrower,
                debt = %vault.debt,
                debt_in_underlying = %vault.debt_in_underlying,
                "Found under-collateralized borrower, triggering liquidation",
            );

            // Craft the function arguments for UniswapV2Pair and HifiFlashSwap.
            let amount0_out = U256::zero();
            let amount1_out = vault.debt_in_underlying;
            let to = self.hifi_flash_swap_address;
            let data = abi::encode(&(*fy_token, *borrower, self.min_profit).into_tokens());

            // Call the Uniswap `swap` function that optimistically lets us borrow the underlying and makes
            // a callback to the HifiFlashSwap contract, which will finally execute the liquidation.
            let mut contract_call = self
                .uniswap_v2_pair
                .swap(amount0_out, amount1_out, to, data)
                .gas_price(gas_price)
                .block(BlockNumber::Pending);

            // Set the nonce for the liquidator account manually, because of this bug in ethers:
            // https://github.com/gakonst/ethers-rs/issues/172
            let client = self.uniswap_v2_pair.client();
            let nonce = client
                .get_transaction_count(self.liquidator_address, Some(BlockNumber::Pending))
                .await
                .map_err(ContractError::MiddlewareError)?;
            contract_call.tx.nonce = Some(nonce);

            // Broadcast the transaction for the first time.
            let pending_tx_request: TransactionRequest = contract_call.tx.clone();
            match contract_call.send().await {
                Ok(pending_tx) => {
                    let pending_tx_hash: TxHash = *pending_tx;
                    trace!(
                        pending_tx_hash = %pending_tx_hash,
                        fy_token = %fy_token,
                        borrower = %borrower,
                        "Submitted liquidation"
                    );
                    self.insert_pending_tx_tuple(*fy_token, *borrower, (pending_tx_request, pending_tx_hash, now));
                }
                Err(err) => {
                    self.handle_reverted_tx(err, "Tx reverted with error");
                }
            };
        }

        Ok(())
    }
}

/// Private methods for the Liquidator struct.
impl<M: Middleware> Liquidator<M> {
    fn get_pending_tx_tuple(&self, fy_token: &Address, borrower: &Address) -> Option<&PendingTransactionTuple> {
        if let Some(inner_hash_map) = self.pending_tx_tuples.get(fy_token) {
            inner_hash_map.get(borrower)
        } else {
            None
        }
    }

    fn handle_reverted_tx(&self, err: ContractError<M>, description: &str) {
        let err = err.to_string();
        if err.contains("ERR_INSUFFICIENT_PROFIT") {
            debug!("Liquidation not profitable.");
        } else if err.contains("ERR_INSUFFICIENT_LOCKED_COLLATERAL") {
            debug!("Collateral price has fallen so hard that not enough collateral can be clutched.");
        } else if err.contains("UniswapV2: INSUFFICIENT_LIQUIDITY") {
            debug!("Insufficient liquidity in Uniswap.");
        } else {
            error!("{}: {}", description, err);
        }
    }

    /// Initialize the borrower-to-pending-tx hash map it it doesn't exist and insert the pending tx.
    fn insert_pending_tx_tuple(&mut self, fy_token: Address, borrower: Address, pending_tx: PendingTransactionTuple) {
        if let Some(inner_hash_map) = self.pending_tx_tuples.get_mut(&fy_token) {
            inner_hash_map.entry(borrower).or_insert(pending_tx);
        } else {
            let mut inner_hash_map = HashMap::<Address, PendingTransactionTuple>::new();
            inner_hash_map.insert(borrower, pending_tx);
            self.pending_tx_tuples.insert(fy_token, inner_hash_map);
        }
    }

    /// Remove the pending tx from the borrower-to-pending-tx hash map. Also remove the hash map itself
    /// if the hash map has become empty.
    fn remove_pending_tx_tuple(&mut self, fy_token: &Address, borrower: &Address) {
        let inner_hash_map = self
            .pending_tx_tuples
            .get_mut(fy_token)
            .expect("Inner hash map must exist when receipt was found on pending transaction.");
        inner_hash_map.remove(borrower);
        if inner_hash_map.is_empty() {
            self.pending_tx_tuples.remove(fy_token);
        }
    }
}
