use crate::{
    liquidations::Liquidator,
    vaults::{Vault, VaultsContainer},
    EthersResult,
};

use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
};
use tracing::debug_span;

#[derive(Deserialize, Default, Serialize)]
/// The state which is stored in our logs on disk.
pub struct State {
    /// The last observed block.
    last_block: U64,
    /// The vaults being monitored.
    vaults: HashMap<Address, HashMap<Address, Vault>>,
}

/// The sentinel monitors the blockchain for liquidation opportunities.
#[allow(dead_code)]
pub struct Sentinel<M> {
    client: Arc<M>,
    last_block: U64,
    liquidator: Liquidator<M>,
    vaults_container: VaultsContainer<M>,
}

/// Public methods for the Sentinel struct.
impl<M: Middleware> Sentinel<M> {
    /// Instantiates the sentinel and the inner liquidator and vaults container.
    /// `state` should be provided if there is data that should be taken into account from a previous run.
    pub async fn new(
        balance_sheet: Address,
        client: Arc<M>,
        fy_tokens: Vec<Address>,
        liquidator: Liquidator<M>,
        multicall: Option<Address>,
        state: Option<State>,
    ) -> EthersResult<Sentinel<M>, M> {
        let (last_block, vaults) = match state {
            Some(state) => (state.last_block, state.vaults),
            None => (U64::zero(), HashMap::new()),
        };
        let vaults_container = VaultsContainer::new(balance_sheet, client.clone(), fy_tokens, multicall, vaults).await;
        Ok(Self {
            client,
            last_block,
            liquidator,
            vaults_container,
        })
    }

    /// Runs the liquidation business logic for the specified block.
    async fn on_block(&mut self, block_number: U64) -> EthersResult<(), M> {
        // Get the gas price - TODO: Replace with gas price oracle
        let gas_price = self
            .client
            .get_gas_price()
            .await
            .map_err(ContractError::MiddlewareError)?;

        // 1. Check if our transactions have been mined.
        self.liquidator.remove_pending_tx_tuple_or_bump_gas_price().await?;

        // 2. Update our dataset with the new block's data.
        self.vaults_container
            .update_vaults(self.client.clone(), self.last_block, block_number)
            .await?;

        // 3. Trigger the liquidation for any under-collateralized borrowers.
        self.liquidator
            .trigger(gas_price, self.vaults_container.get_vaults_iterator())
            .await?;

        Ok(())
    }

    pub async fn run(&mut self, log_file_name: PathBuf, start_block: Option<U64>) -> EthersResult<(), M> {
        // Create the initial list of borrowers from the "start_block", if provided
        if let Some(start_block) = start_block {
            self.last_block = start_block;
        }

        let watcher = self.client.clone();
        let mut on_block = watcher
            .watch_blocks()
            .await
            .map_err(ContractError::MiddlewareError)?
            .stream();

        let mut db_file: Option<File> = None;
        while on_block.next().await.is_some() {
            let block_number = self
                .client
                .get_block_number()
                .await
                .map_err(ContractError::MiddlewareError)?;

            // On every 10th block we open a new file handler to dump the latest state.
            // TODO: we should have a database connection instead here ...
            if block_number % 10 == U64::zero() {
                db_file = Some(
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(&log_file_name)
                        .unwrap(),
                );
            }

            let span = debug_span!("eloop", block = %block_number);
            let _enter = span.enter();

            // Run the liquidation logic for this block.
            self.on_block(block_number).await?;

            // Update the last block.
            self.last_block = block_number;

            // Log once every 10 blocks.
            if let Some(db_file) = db_file.take() {
                self.log_state(db_file);
            }
        }

        Ok(())
    }
}

/// Public methods for the Sentinel struct.
impl<M: Middleware> Sentinel<M> {
    fn log_state<W: Write>(&self, db_file: W) {
        serde_json::to_writer(
            db_file,
            &State {
                last_block: self.last_block,
                vaults: self.vaults_container.vaults.clone(),
            },
        )
        .unwrap();
    }
}
