//! Liquidator Module
//!
//! This module is responsible for triggering liquidations.
use crate::{escalator::GeometricGasPrice, vault::Vault, HifiResult};

use ethers::{
    core::abi::{self, Tokenize},
    prelude::*,
};
use hifi_liquidator_bindings::{FyToken, UniswapV2Pair};
use std::{collections::HashMap, sync::Arc, time::Instant};
use tracing::{debug, info, trace};

/// TxRequest / Hash/ Submitted at time
type PendingTransactionTuple = (TransactionRequest, TxHash, Instant);

pub struct Liquidator<M> {
    fy_token: FyToken<M>,
    gas_escalator: GeometricGasPrice,
    hifi_flash_swap: Address,
    min_profit: U256,
    multicall: Multicall<M>,
    pending_liquidations: HashMap<Address, PendingTransactionTuple>,
    uniswap_v2_pair: UniswapV2Pair<M>,
}

impl<M> Liquidator<M>
where
    M: Middleware,
{
    /// Constructor
    pub async fn new(
        client: Arc<M>,
        fy_token: Address,
        hifi_flash_swap: Address,
        gas_escalator: GeometricGasPrice,
        min_profit: U256,
        multicall: Option<Address>,
        uniswap_v2_pair: Address,
    ) -> Self {
        let multicall = Multicall::new(client.clone(), multicall)
            .await
            .expect("Could not initialize Multicall");
        Self {
            fy_token: FyToken::new(fy_token, client.clone()),
            hifi_flash_swap,
            gas_escalator,
            min_profit,
            multicall,
            pending_liquidations: HashMap::new(),
            uniswap_v2_pair: UniswapV2Pair::new(uniswap_v2_pair, client.clone()),
        }
    }

    /// Checks if any transactions which have been submitted are mined and removes them if they
    /// were successful. Otherwise, it bumps their gas price.
    pub async fn remove_or_bump(&mut self) -> HifiResult<(), M> {
        let now = Instant::now();

        // Check all pending liquidations.
        self.remove_or_bump_inner(now).await?;

        Ok(())
    }

    async fn remove_or_bump_inner(&mut self, now: Instant) -> HifiResult<(), M> {
        let client = self.fy_token.client();

        for (account, pending_tx) in self.pending_liquidations.iter_mut() {}

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the BalanceSheet.
    /// It does this with capital sourced from Uniswap V2.
    pub async fn trigger_liquidations(
        &mut self,
        gas_price: U256,
        vaults: impl Iterator<Item = (&Address, &Vault)>,
    ) -> HifiResult<(), M> {
        debug!("Checking for under-collateralized positions...");

        let now = Instant::now();

        for (borrower, vault) in vaults {
            // Only iterate over (fyToken, borrower) pairs that do not have pending liquidations.
            if let Some(pending_tx) = self.pending_liquidations.get(&borrower) {
                trace!(pending_tx = ?pending_tx, borrower = ?borrower, "Liquidation tx not confirmed yet");
                continue;
            }

            // Skip vaults that have no outstanding debt.
            if vault.debt.is_zero() {
                continue;
            }

            if vault.is_underwater {
                info!(
                    borrower = ?borrower,
                    debt = %vault.debt,
                    "Found under-collateralized borrower. Triggering liquidation.",
                );

                // Craft the HifiFlashSwap contract's arguments.
                let args = abi::encode(&(*borrower, self.min_profit).into_tokens());

                // Call the Uniswap `swap` function which will optimistically let us borrow the underlying and
                // make a callback to the HifiFlashSwap contract, which will execute the liquidation.
                // TODO: convert `debt` from fyUSDC to USDC decimals.
                let contract_call = self
                    .uniswap_v2_pair
                    .swap(0.into(), vault.debt, self.hifi_flash_swap, args)
                    .gas_price(gas_price)
                    .block(BlockNumber::Pending);
                let pending_tx = contract_call.send().await?;

                let tx_request: TransactionRequest = contract_call.tx.clone();
                let tx_hash: TxHash = *pending_tx;
                trace!(pending_tx = ?pending_tx, borrower = ?borrower, "Submitted liquidation");
                self.pending_liquidations
                    .entry(*borrower)
                    .or_insert((tx_request, tx_hash, now));
            }
        }

        Ok(())
    }
}
