use crate::{
    escalator::GeometricGasPrice,
    liquidator::Liquidator,
    vault::{Vault, VaultContainer},
    HifiLiquidatorResult,
};

use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Serialize, Deserialize, Default)]
/// The state which is stored in our logs on disk.
pub struct State {
    /// The last observed block.
    last_block: u64,
    /// The vaults being monitored.
    vaults: HashMap<Address, HashMap<Address, Vault>>,
}

/// The sentinel monitors the blockchain for liquidation opportunities.
#[allow(dead_code)]
pub struct Sentinel<M> {
    client: Arc<M>,
    last_block: u64,
    liquidator: Liquidator<M>,
    vault_container: VaultContainer<M>,
}

impl<M> Sentinel<M>
where
    M: Middleware,
{
    /// Instantiates the sentinel. `state` should be passed if there is previous data that should
    /// be taken into account from a previous run.
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        balance_sheet: Address,
        client: Arc<M>,
        gas_escalator: GeometricGasPrice,
        hifi_flash_swap: Address,
        multicall: Option<Address>,
        min_profit: U256,
        state: Option<State>,
        uniswap_v2_pair: Address,
    ) -> HifiLiquidatorResult<Sentinel<M>, M> {
        let (last_block, vaults) = match state {
            Some(state) => (state.last_block, state.vaults),
            None => (0, HashMap::new()),
        };

        let vault_container = VaultContainer::new(balance_sheet, client.clone(), multicall, vaults).await;

        let liquidator = Liquidator::new(
            client.clone(),
            gas_escalator,
            hifi_flash_swap,
            min_profit,
            uniswap_v2_pair,
        );

        Ok(Self {
            client,
            last_block,
            liquidator,
            vault_container,
        })
    }
}
