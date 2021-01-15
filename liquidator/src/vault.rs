//! Borrowers
//!
//! This module is responsible for keeping track of the Hifi users that have open
//! positions and monitoring their debt healthiness.
use crate::HifiLiquidatorResult;

use ethers::prelude::*;
use hifi_liquidator_bindings::BalanceSheet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::debug_span;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A borrower's vault
pub struct Vault {
    /// The borrower's total debt. Produced by calling `getVaultDebt` on the Fintroller.
    pub debt: U256,

    /// Is the vault liquidatable? Obtained by calling `isAccountUnderwater` on the Fintroller.
    pub is_underwater: bool,

    /// The borrower's currently locked collateral. Produced by calling `getVaultLockedCollateral`
    /// on the Fintroller.
    pub locked_collateral: U256,
}

#[derive(Clone)]
pub struct VaultContainer<M> {
    /// The BalanceSheet smart contract
    pub balance_sheet: BalanceSheet<M>,

    /// We use Multicall to batch together calls and have reduced stress on our RPC endpoint.
    multicall: Multicall<M>,

    /// Mapping of the Hifi accounts that have taken loans and might be liquidatable. The first address
    /// is the FyToken, the second the borrower's account.
    pub vaults: HashMap<Address, HashMap<Address, Vault>>,
}

impl<M: Middleware> VaultContainer<M> {
    /// Constructor
    pub async fn new(
        balance_sheet: Address,
        client: Arc<M>,
        multicall: Option<Address>,
        vaults: HashMap<Address, HashMap<Address, Vault>>,
    ) -> Self {
        let multicall = Multicall::new(client.clone(), multicall)
            .await
            .expect("Could not initialize Multicall");
        VaultContainer {
            balance_sheet: BalanceSheet::new(balance_sheet, client),
            multicall,
            vaults,
        }
    }

    /// Indexes any new vaults which may have been opened since we last made this call. Then, it proceeds
    /// to get the latest account details for each user.
    pub async fn update_vaults(&mut self, from_block: U64, to_block: U64) -> HifiLiquidatorResult<(), M> {
        let span = debug_span!("monitoring");
        let _enter = span.enter();

        // TODO: index the "FreeCollateral" event to remove the empty vaults from the cache.
        let new_vaults: Vec<(Address, Address)> = self
            .balance_sheet
            .open_vault_filter()
            .from_block(from_block)
            .to_block(to_block)
            .query()
            .await?
            .into_iter()
            .map(|log| (log.fy_token, log.borrower))
            .collect::<Vec<_>>();

        for vault_tuple in new_vaults {
            let vault = self.get_vault(vault_tuple.0, vault_tuple.1).await?;

            // Either initialize the inner HashMap or insert the transaction in the existing one.
            if let Some(inner_hash_map) = self.vaults.get_mut(&vault_tuple.0) {
                inner_hash_map.insert(vault_tuple.1, vault.clone());
            } else {
                let mut inner_hash_map = HashMap::<Address, Vault>::new();
                inner_hash_map.insert(vault_tuple.1, vault.clone());
                self.vaults.insert(vault_tuple.0, inner_hash_map);
            }
        }

        Ok(())
    }

    /// Updates the vault's details by calling:
    /// 1. getVaultDebt
    /// 2. isAccountUnderwater
    /// 3. getVaultLockedCollateral
    pub async fn get_vault(&mut self, fy_token: Address, borrower: Address) -> HifiLiquidatorResult<Vault, M> {
        let debt = self.balance_sheet.get_vault_debt(fy_token, borrower);
        let is_underwater = self.balance_sheet.is_account_underwater(fy_token, borrower);
        let locked_collateral = self.balance_sheet.get_vault_locked_collateral(fy_token, borrower);

        // Batch the calls together.
        let multicall = self
            .multicall
            .clear_calls()
            .add_call(debt)
            .add_call(is_underwater)
            .add_call(locked_collateral);
        let (debt, is_underwater, locked_collateral) = multicall.call().await?;

        Ok(Vault {
            debt,
            is_underwater,
            locked_collateral,
        })
    }
}
