//! Liquidator Module
//!
//! This module is responsible for triggering liquidations.
use crate::{escalator::GeometricGasPrice, vault::Vault, HifiResult};

use ethers::{
    core::abi::{self, Tokenize},
    prelude::*,
};
use hifi_liquidator_bindings::{FyToken, UniswapV2Pair};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, sync::Arc, time::Instant};
use tracing::{debug, debug_span, error, info, trace};

#[derive(Clone)]
pub struct Liquidator<M> {
    fy_token: FyToken<M>,
    gas_escalator: GeometricGasPrice,
    hifi_flash_swap: Address,
    uniswap_v2_pair: UniswapV2Pair<M>,

    /// The minimum profit to be extracted per liquidation
    min_profit: U256,

    /// We use multicall to batch together calls and have reduced stress on
    /// our RPC endpoint
    multicall: Multicall<M>,

    pending_liquidations: HashMap<Address, PendingTransaction>,
}

/// Tx / Hash/ Submitted at time
type PendingTransaction = (TransactionRequest, TxHash, Instant);

impl<M: Middleware> Liquidator<M> {
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
            .expect("Could not initialize multicall");
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

    /// Checks if any transactions which have been submitted are mined, removes
    /// them if they were successful, otherwise bumps their gas price
    pub async fn remove_or_bump(&mut self) -> Result<(), M> {
        let now = Instant::now();

        // Check all the pending liquidations
        self.remove_or_bump_inner(now).await?;

        Ok(())
    }

    async fn remove_or_bump_inner(&mut self, now: Instant) -> Result<(), M> {
        let client = self.fy_token.client();

        let pending_txs = &mut self.pending_liquidations;

        for (addr, pending_tx) in pending_txs.clone().into_iter() {
            debug_assert!(pending_tx.0.gas_price.is_some(), "Gas price must be set in pending txs");

            // Get the receipt and check inclusion, or bump its gas price
            let receipt = client
                .get_transaction_receipt(pending_tx.1)
                .await
                .map_err(ContractError::MiddlewareError)?;

            if let Some(receipt) = receipt {
                pending_txs.remove(&addr);
                let status = if receipt.status == Some(1.into()) {
                    "success"
                } else {
                    "fail"
                };
                trace!(tx_hash = ?pending_tx.1, gas_used = %receipt.gas_used.unwrap_or_default(), user = ?addr, status = status, "Confirmed");
            } else {
                // Get the new gas price based on how much time passed since the tx was last broadcast.
                let new_gas_price = self.gas_escalator.get_gas_price(
                    pending_tx.0.gas_price.expect("Gas price must be set"),
                    now.duration_since(pending_tx.2).as_secs(),
                );

                let replacement_tx = pending_txs
                    .get_mut(&addr)
                    .expect("tx will always be found since we're iterating over the map");

                // Bump the gas price.
                replacement_tx.0.gas_price = Some(new_gas_price);

                // Rebroadcast (TODO: Can we avoid cloning?).
                replacement_tx.1 = client
                    .send_transaction(replacement_tx.0.clone(), None)
                    .await
                    .map_err(ContractError::MiddlewareError)?;

                trace!(tx_hash = ?pending_tx.1, new_gas_price = %new_gas_price, user = ?addr, "Replaced");
            }
        }

        Ok(())
    }

    /// Sends a bid for any of the liquidation auctions.
    pub async fn buy_opportunities(&mut self, from_block: U64, to_block: U64, gas_price: U256) -> Result<(), M> {
        let all_users = {
            let liquidations = self
                .liquidations
                .liquidation_filter()
                .from_block(from_block)
                .to_block(to_block)
                .query()
                .await?;
            let new_users = liquidations.iter().map(|log| log.user).collect::<Vec<_>>();
            merge(new_users, &self.auctions)
        };

        for user in all_users {
            self.buy(user, Instant::now(), gas_price).await?;
        }

        Ok(())
    }

    /// Tries to buy the collateral associated with a user's liquidation auction
    /// via a flashloan funded by UniswapV2Pair's DAI/WETH pair.
    async fn buy(&mut self, user: Address, now: Instant, gas_price: U256) -> Result<(), M> {
        // only iterate over users that do not have active auctions
        if let Some(pending_tx) = self.pending_auctions.get(&user) {
            trace!(tx_hash = ?pending_tx.1, user = ?user, "bid not confirmed yet");
            return Ok(());
        }

        // Get the vault's info
        let vault = self.get_vault(user).await?;
        // Skip auctions which do not have any outstanding debt
        if vault.debt == 0 {
            return Ok(());
        }

        if self.auctions.insert(user, vault.clone()).is_none() {
            debug!(user = ?user, vault = ?vault, "new auction");
        }
        let span = debug_span!("buying", user = ?user, auction_start = %vault.started, auction_end = %(vault.started + 3600), debt = %vault.debt);
        let _enter = span.enter();

        // Craft the flashloan contract's arguments
        let args = abi::encode(&(user, self.min_profit).into_tokens());

        // Calls UniswapV2Pair's `swap` function which will optimistically let us
        // borrow the debt, which will then make a callback to the flashloan
        // contract which will execute the liquidation
        let call = self
            .uniswap
            .swap(vault.debt.into(), 0.into(), self.flashloan, args)
            .gas_price(gas_price)
            .block(BlockNumber::Pending);

        let tx = call.tx.clone();

        match call.send().await {
            Ok(hash) => {
                // record the tx
                trace!(tx_hash = ?hash, "Submitted buy order");
                self.pending_auctions.entry(user).or_insert((tx, hash, now));
            }
            Err(err) => {
                let err = err.to_string();
                if err.contains("NOT_ENOUGH_PROFIT") {
                    let price = self.liquidations.price(user).call().await?;
                    debug!(price = %price, "Auction not yet profitable via UniswapV2Pair flash swaps.");
                } else if err.contains("Below dust") {
                    debug!("Proceeds are below the dust limit, ignoring..");
                } else {
                    error!("Error: {}", err);
                }
            }
        };

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    pub async fn trigger_liquidations(
        &mut self,
        borrowers: impl Iterator<Item = (&Address, &Borrower)>,
        gas_price: U256,
    ) -> Result<(), M> {
        debug!("checking for undercollateralized positions...");

        let now = Instant::now();

        for (user, details) in borrowers {
            // only iterate over users that do not have pending liquidations
            if let Some(pending_tx) = self.pending_liquidations.get(&user) {
                trace!(tx_hash = ?pending_tx.1, user = ?user, "liquidation not confirmed yet");
                continue;
            }

            if !details.is_collateralized {
                info!(
                    user = ?user,
                    debt_dai = %details.debt,
                    max_debt_dai = %details.max_borrowing_power,
                    "found undercollateralized user. triggering liquidation",
                );

                // Send the tx and track it
                let call = self.liquidations.liquidate(*user).gas_price(gas_price);
                let tx = call.tx.clone();
                let tx_hash = call.send().await?;
                trace!(tx_hash = ?tx_hash, user = ?user, "Submitted liquidation");
                self.pending_liquidations.entry(*user).or_insert((tx, tx_hash, now));
            }
        }

        Ok(())
    }

    async fn get_vault(&mut self, user: Address) -> Result<Auction, M> {
        let vault = self.liquidations.vaults(user);
        let timestamp = self.liquidations.liquidations(user);

        let multicall = self.multicall.clear_calls().add_call(vault).add_call(timestamp);
        let (vault, timestamp): ((u128, u128), U256) = multicall.call().await?;

        Ok(Auction {
            started: timestamp,
            collateral: vault.0,
            debt: vault.1,
        })
    }
}
