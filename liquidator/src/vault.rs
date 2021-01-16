//! Borrowers
//!
//! This module is responsible for keeping track of the Hifi users that have open
//! positions and monitoring their debt healthiness.
use crate::EthersResult;

use ethers::prelude::*;
use hifi_liquidator_bindings::{BalanceSheet, FyToken};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::debug_span;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A borrower's vault
pub struct Vault {
    /// The borrower's total debt. Obtained by calling `getVaultDebt` on the BalanceSheet.
    /// NOTE: the debt here is recorded in terms of underlying, but the Hifi protocol records
    /// everything in terms of fyTokens.
    pub debt: U256,

    /// Is the vault liquidatable? Obtained by calling `isAccountUnderwater` on the BalanceSheet.
    pub is_account_underwater: bool,

    /// The borrower's currently locked collateral. Obtained by calling `getVaultLockedCollateral`
    /// on the BalanceSheet.
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

    pub fn get_vaults_iterator(&self) -> impl Iterator<Item = (&Address, &Address, &Vault)> {
        let mut vaults_iterator: Vec<(&Address, &Address, &Vault)> = vec![];

        for (fy_token, borrower_to_vault_hash_map) in self.vaults.iter() {
            for (borrower, vault) in borrower_to_vault_hash_map.iter() {
                vaults_iterator.push((fy_token, borrower, vault));
            }
        }

        vaults_iterator.into_iter()
    }

    /// Updates the vault's details by calling:
    ///
    /// 1. getVaultDebt
    /// 2. isAccountUnderwater
    /// 3. getVaultLockedCollateral
    /// 4. underlyingPrecisionScalar
    pub async fn get_vault(&mut self, client: Arc<M>, fy_token: Address, borrower: Address) -> EthersResult<Vault, M> {
        let debt_call = self.balance_sheet.get_vault_debt(fy_token, borrower);
        let is_account_underwater_call = self.balance_sheet.is_account_underwater(fy_token, borrower);
        let locked_collateral_call = self.balance_sheet.get_vault_locked_collateral(fy_token, borrower);

        // TODO: cache these instances of FyToken.
        let fy_token = FyToken::new(fy_token, client);
        let underlying_precision_scalar_call = fy_token.underlying_precision_scalar();

        // Batch the calls together.
        let multicall: &mut Multicall<M> = self
            .multicall
            .clear_calls()
            .add_call(debt_call)
            .add_call(is_account_underwater_call)
            .add_call(locked_collateral_call)
            .add_call(underlying_precision_scalar_call);
        let (debt, is_account_underwater, locked_collateral, underlying_precision_scalar): (U256, bool, U256, U256) =
            multicall.call().await?;

        // Scale the debt down by the underlying precision scalar. For example, USDC has 6 decimals, so the debt is
        // scaled from 1e20 (100 fYUSDC) to 1e8 (100 USDC).
        println!("pre_debt: {}", debt);
        let debt = debt / underlying_precision_scalar;
        println!("post_debt: {}", debt);

        Ok(Vault {
            debt,
            is_account_underwater,
            locked_collateral,
        })
    }

    /// Indexes any new vaults which may have been opened since we last made this call. Then, it proceeds
    /// to get the latest account details for each user.
    pub async fn update_vaults(&mut self, client: Arc<M>, from_block: U64, to_block: U64) -> EthersResult<(), M> {
        let span = debug_span!("monitoring");
        let _enter = span.enter();

        let new_vault_tuples: Vec<(Address, Address)> = self
            .balance_sheet
            .open_vault_filter()
            .from_block(from_block)
            .to_block(to_block)
            .query()
            .await?
            .into_iter()
            .map(|event_log| (event_log.fy_token, event_log.borrower))
            .collect::<Vec<_>>();

        for vault_tuple in new_vault_tuples.iter() {
            let vault = self.get_vault(client.clone(), vault_tuple.0, vault_tuple.1).await?;

            // Either initialize the inner HashMap or insert the transaction in the existing one.
            if let Some(borrower_to_vault_hash_map) = self.vaults.get_mut(&vault_tuple.0) {
                borrower_to_vault_hash_map.insert(vault_tuple.1, vault.clone());
            } else {
                let mut borrower_to_vault_hash_map = HashMap::<Address, Vault>::new();
                borrower_to_vault_hash_map.insert(vault_tuple.1, vault.clone());
                self.vaults.insert(vault_tuple.0, borrower_to_vault_hash_map);
            }
        }

        Ok(())
    }
}
