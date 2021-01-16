//! Liquidator Module
//!
//! This module is responsible for triggering liquidations.
use crate::{vault::Vault, EthersResult};

use ethers::{
    core::abi::{self, Tokenize},
    middleware::gas_escalator::{GasEscalator, GeometricGasPrice},
    prelude::*,
};
use hifi_liquidator_bindings::UniswapV2Pair;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tracing::{debug, info, trace};

/// TxRequest / Hash/ Submitted at time.
type PendingTransactionTuple = (TransactionRequest, TxHash, Instant);

pub struct Liquidator<M> {
    gas_escalator: GeometricGasPrice,
    hifi_flash_swap: Address,
    min_profit: U256,
    pending_txs: HashMap<Address, HashMap<Address, PendingTransactionTuple>>,
    uniswap_v2_pair: UniswapV2Pair<M>,
}

/// Public methods for the Liquidator struct.
impl<M: Middleware> Liquidator<M> {
    /// Constructor
    pub fn new(
        client: Arc<M>,
        gas_escalator: GeometricGasPrice,
        hifi_flash_swap: Address,
        min_profit: U256,
        uniswap_v2_pair: Address,
    ) -> Self {
        Self {
            hifi_flash_swap,
            gas_escalator,
            min_profit,
            pending_txs: HashMap::new(),
            uniswap_v2_pair: UniswapV2Pair::new(uniswap_v2_pair, client.clone()),
        }
    }

    /// Checks if any transactions which have been submitted are mined and removes them if they
    /// were successful. Otherwise, it bumps their gas price.
    pub async fn remove_pending_tx_or_bump_gas_price(&mut self) -> EthersResult<(), M> {
        let now = Instant::now();

        for (fy_token, borrower_to_pending_tx_hash_map) in self.pending_txs.clone().into_iter() {
            for (borrower, pending_tx) in borrower_to_pending_tx_hash_map {
                self.remove_pending_tx_or_bump_gas_price_internal(now, &fy_token, &borrower, &pending_tx)
                    .await?;
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
            // Only iterate over (fy_token, borrower, vault) tuples that do not have pending liquidations.
            if let Some(pending_tx) = self.get_pending_tx(&fy_token, &borrower) {
                trace!(
                    pending_tx = ?pending_tx,
                    fy_token = ?fy_token,
                    borrower = ?borrower,
                    "Liquidation tx not confirmed yet."
                );
                continue;
            }

            // Skip vaults that are not underwater.
            if !vault.is_account_underwater || vault.debt.is_zero() {
                continue;
            }

            // TODO: calculate the maximum borrowing power of the user.
            info!(
                fy_token = ?fy_token,
                borrower = ?borrower,
                debt = %vault.debt,
                "Found under-collateralized borrower. Triggering liquidation.",
            );

            // Craft the HifiFlashSwap contract's arguments.
            let amount0_out = U256::zero();
            let amount1_out = vault.debt;
            let to = self.hifi_flash_swap;
            let data = abi::encode(&(*fy_token, *borrower, self.min_profit).into_tokens());

            // Call the Uniswap `swap` function that optimistically lets us borrow the underlying and makes
            // a callback to the HifiFlashSwap contract, which will finally execute the liquidation.
            let contract_call = self
                .uniswap_v2_pair
                .swap(amount0_out, amount1_out, to, data)
                .gas_price(gas_price)
                .block(BlockNumber::Pending);
            let pending_tx = contract_call.send().await?;
            let pending_tx_request: TransactionRequest = contract_call.tx.clone();
            let pending_tx_hash: TxHash = *pending_tx;
            self.insert_pending_tx_in_hash_map(fy_token, borrower, (pending_tx_request, pending_tx_hash, now));
            trace!(
                pending_tx = ?pending_tx,
                fy_token = ?fy_token,
                borrower = ?borrower,
                "Submitted liquidation."
            );
        }

        Ok(())
    }
}

/// Private methods for the Liquidator struct.
impl<M: Middleware> Liquidator<M> {
    fn get_pending_tx(&self, fy_token: &Address, borrower: &Address) -> Option<&PendingTransactionTuple> {
        if let Some(borrower_to_pending_tx_hash_map) = self.pending_txs.get(fy_token) {
            borrower_to_pending_tx_hash_map.get(borrower)
        } else {
            None
        }
    }

    /// Initialize the borrower-to-pending-tx hash map it it doesn't exist and insert the pending tx.
    fn insert_pending_tx_in_hash_map(
        &mut self,
        fy_token: &Address,
        borrower: &Address,
        pending_tx: PendingTransactionTuple,
    ) {
        if let Some(borrower_to_pending_tx_hash_map) = self.pending_txs.get_mut(fy_token) {
            borrower_to_pending_tx_hash_map.entry(*borrower).or_insert(pending_tx);
        } else {
            let mut borrower_to_pending_tx_hash_map = HashMap::<Address, PendingTransactionTuple>::new();
            borrower_to_pending_tx_hash_map.entry(*borrower).or_insert(pending_tx);
            self.pending_txs.insert(*fy_token, borrower_to_pending_tx_hash_map);
        }
    }

    /// Remove the pending tx from the borrower-to-pending-tx hash map. Also remove the hash map itself
    /// if the hash map has become empty.
    fn remove_pending_tx_from_hash_map(&mut self, fy_token: &Address, borrower: &Address) {
        let borrower_to_pending_tx_hash_map = self
            .pending_txs
            .get_mut(fy_token)
            .expect("Inner hash map must exist when receipt was found on pending transaction");
        borrower_to_pending_tx_hash_map.remove(borrower);
        if borrower_to_pending_tx_hash_map.is_empty() {
            self.pending_txs.remove(fy_token);
        }
    }

    /// Remove the pending tx from the hash map if we got a receipt from the blockchain, otherwise bump
    /// the gas price.
    async fn remove_pending_tx_or_bump_gas_price_internal(
        &mut self,
        now: Instant,
        fy_token: &Address,
        borrower: &Address,
        pending_tx: &PendingTransactionTuple,
    ) -> EthersResult<(), M> {
        debug_assert!(pending_tx.0.gas_price.is_some(), "Gas price must be set in pending txs");

        let client = self.uniswap_v2_pair.client();
        let receipt = client
            .get_transaction_receipt(pending_tx.1)
            .await
            .map_err(ContractError::MiddlewareError)?;

        if let Some(receipt) = receipt {
            self.remove_pending_tx_from_hash_map(fy_token, borrower);

            let status = if receipt.status == Some(1.into()) {
                "Success"
            } else {
                "Fail"
            };

            trace!(
                tx_hash = ?pending_tx.1,
                gas_used = %receipt.gas_used.unwrap_or_default(),
                fy_token = ?fy_token,
                borrower = ?borrower,
                status = status,
                "Confirmed"
            );
        } else {
            // Get the new gas price based on how much time passed since the tx was last broadcast.
            let new_gas_price = self.gas_escalator.get_gas_price(
                pending_tx.0.gas_price.expect("Gas price must be set"),
                now.duration_since(pending_tx.2).as_secs(),
            );

            let replacement_tx = self
                .pending_txs
                .get_mut(&fy_token)
                .expect("Inner hash map will always be found since we're iterating over the map")
                .get_mut(&borrower)
                .expect("Pending tx will always be found since we're iterating over the map");

            // Bump the gas price.
            replacement_tx.0.gas_price = Some(new_gas_price);

            // Rebroadcast (TODO: Can we avoid cloning?).
            replacement_tx.1 = *client
                .send_transaction(replacement_tx.0.clone(), None)
                .await
                .map_err(ContractError::MiddlewareError)?;

            trace!(
                tx_hash = ?pending_tx.1,
                new_gas_price = %new_gas_price,
                fy_token = ?fy_token,
                borrower = ?borrower,
                "Replaced"
            );
        }
        Ok(())
    }
}
