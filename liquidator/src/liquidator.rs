//! Liquidator Module
//!
//! This module is responsible for triggering liquidations.
use crate::{escalator::GeometricGasPrice, vault::Vault, HifiLiquidatorResult};

use ethers::{
    core::abi::{self, Tokenize},
    prelude::*,
};
use hifi_liquidator_bindings::UniswapV2Pair;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tracing::{debug, info, trace};

/// TxRequest / Hash/ Submitted at time
type PendingTransactionTuple = (TransactionRequest, TxHash, Instant);

pub struct Liquidator<M> {
    gas_escalator: GeometricGasPrice,
    hifi_flash_swap: Address,
    min_profit: U256,
    pending_liquidations: HashMap<Address, HashMap<Address, PendingTransactionTuple>>,
    uniswap_v2_pair: UniswapV2Pair<M>,
}

impl<M> Liquidator<M>
where
    M: Middleware,
{
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
            pending_liquidations: HashMap::new(),
            uniswap_v2_pair: UniswapV2Pair::new(uniswap_v2_pair, client),
        }
    }

    /// Checks if any transactions which have been submitted are mined and removes them if they
    /// were successful. Otherwise, it bumps their gas price.
    pub async fn remove_or_bump(&mut self) -> HifiLiquidatorResult<(), M> {
        let now = Instant::now();

        // Check all pending liquidations.
        self.remove_or_bump_inner(now).await?;

        Ok(())
    }

    async fn remove_or_bump_inner(&mut self, now: Instant) -> HifiLiquidatorResult<(), M> {
        let client = self.uniswap_v2_pair.client();

        let pending_txs = &mut self.pending_liquidations;

        for (fy_token, inner_hash_map) in pending_txs.clone().into_iter() {
            for (borrower, pending_tx) in inner_hash_map {
                debug_assert!(pending_tx.0.gas_price.is_some(), "Gas price must be set in pending txs");

                let receipt = client
                    .get_transaction_receipt(pending_tx.1)
                    .await
                    .map_err(ContractError::MiddlewareError)?;

                // Check inclusion, or bump its gas price.
                if let Some(receipt) = receipt {
                    pending_txs.remove(&borrower);

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

                    let replacement_tx = pending_txs
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
            }
        }

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the BalanceSheet.
    /// It does this with capital sourced from Uniswap V2.
    pub async fn trigger_liquidations(
        &mut self,
        gas_price: U256,
        vaults: impl Iterator<Item = (&Address, &Address, &Vault)>,
    ) -> HifiLiquidatorResult<(), M> {
        debug!("Checking for under-collateralized positions...");

        let now = Instant::now();

        for (fy_token, borrower, vault) in vaults {
            // Only iterate over (fy_token, borrower, vault) pairs that do not have pending liquidations.
            if let Some(inner_hash_map) = self.pending_liquidations.get(&fy_token) {
                if let Some(pending_tx) = inner_hash_map.get(&borrower) {
                    trace!(
                        pending_tx = ?pending_tx,
                        fy_token = ?fy_token,
                        borrower = ?borrower,
                        "Liquidation tx not confirmed yet"
                    );
                    continue;
                }
            }

            // Skip vaults that either have no outstanding debt or are not underwater.
            if vault.debt.is_zero() || !vault.is_underwater {
                continue;
            }

            info!(
                fy_token = ?fy_token,
                borrower = ?borrower,
                debt = %vault.debt,
                "Found under-collateralized borrower. Triggering liquidation.",
            );

            // Craft the HifiFlashSwap contract's arguments.
            let data = abi::encode(&(*fy_token, *borrower, self.min_profit).into_tokens());

            // Call the Uniswap `swap` function which will optimistically let us borrow the underlying and
            // make a callback to the HifiFlashSwap contract, which will execute the liquidation.
            // TODO: convert `debt` from fyUSDC to USDC decimals.
            let contract_call = self
                .uniswap_v2_pair
                .swap(0.into(), vault.debt, self.hifi_flash_swap, data)
                .gas_price(gas_price)
                .block(BlockNumber::Pending);
            let pending_tx = contract_call.send().await?;

            let tx_request: TransactionRequest = contract_call.tx.clone();
            let tx_hash: TxHash = *pending_tx;
            trace!(pending_tx = ?pending_tx, borrower = ?borrower, "Submitted liquidation");

            // Either initialize the inner HashMap or insert the transaction in the existing one.
            if let Some(inner_hash_map) = self.pending_liquidations.get_mut(fy_token) {
                inner_hash_map.entry(*borrower).or_insert((tx_request, tx_hash, now));
            } else {
                let mut inner_hash_map = HashMap::<Address, PendingTransactionTuple>::new();
                inner_hash_map.entry(*borrower).or_insert((tx_request, tx_hash, now));
                self.pending_liquidations.insert(*fy_token, inner_hash_map);
            }
        }

        Ok(())
    }
}
