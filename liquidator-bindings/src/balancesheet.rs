#![allow(clippy::all)]
#[rustfmt::skip]

pub use balancesheet_mod::*;
#[allow(clippy::too_many_arguments)]
mod balancesheet_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{parse_abi, Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::Middleware,
    };
    #[doc = "BalanceSheet was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BALANCESHEET_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FintrollerInterface\",\n        \"name\": \"fintroller_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"liquidator\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"clutchedCollateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ClutchCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"DepositCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"FreeCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LockCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OpenVault\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"oldDebt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newDebt\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SetVaultDebt\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"oldAdmin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"TransferAdmin\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"WithdrawCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"_renounceAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"_transferAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"admin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"liquidator\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"clutchCollateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"depositCollateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"fintroller\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract FintrollerInterface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"freeCollateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"repayAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getClutchableCollateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCurrentCollateralizationRatio\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"lockedCollateral\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"debt\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getHypotheticalCollateralizationRatio\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getVault\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getVaultDebt\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getVaultLockedCollateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isAccountUnderwater\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"isBalanceSheet\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isVaultOpen\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"lockCollateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"openVault\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newVaultDebt\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setVaultDebt\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"withdrawCollateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct BalanceSheet<M>(Contract<M>);
    impl<M> std::ops::Deref for BalanceSheet<M> {
        type Target = Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: Middleware> std::fmt::Debug for BalanceSheet<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BalanceSheet)).field(&self.address()).finish()
        }
    }
    impl<'a, M: Middleware> BalanceSheet<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>>(address: T, client: Arc<M>) -> Self {
            let contract = Contract::new(address.into(), BALANCESHEET_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `getVault` (0xbbd7edc5) function"]
        pub fn get_vault(&self, fy_token: Address, borrower: Address) -> ContractCall<M, (U256, U256, U256, bool)> {
            self.0
                .method_hash([187, 215, 237, 197], (fy_token, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_renounceAdmin` (0xbfa25308) function"]
        pub fn renounce_admin(&self) -> ContractCall<M, ()> {
            self.0
                .method_hash([191, 162, 83, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lockCollateral` (0x47a7d107) function"]
        pub fn lock_collateral(&self, fy_token: Address, collateral_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([71, 167, 209, 7], (fy_token, collateral_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawCollateral` (0x350c35e9) function"]
        pub fn withdraw_collateral(&self, fy_token: Address, collateral_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([53, 12, 53, 233], (fy_token, collateral_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isVaultOpen` (0xb4f2a9cc) function"]
        pub fn is_vault_open(&self, fy_token: Address, borrower: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([180, 242, 169, 204], (fy_token, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `openVault` (0x318e7028) function"]
        pub fn open_vault(&self, fy_token: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([49, 142, 112, 40], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAccountUnderwater` (0x9ee91d1f) function"]
        pub fn is_account_underwater(&self, fy_token: Address, borrower: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([158, 233, 29, 31], (fy_token, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getClutchableCollateral` (0x464b0485) function"]
        pub fn get_clutchable_collateral(&self, fy_token: Address, repay_amount: U256) -> ContractCall<M, U256> {
            self.0
                .method_hash([70, 75, 4, 133], (fy_token, repay_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVaultLockedCollateral` (0x1a7a4eb4) function"]
        pub fn get_vault_locked_collateral(&self, fy_token: Address, borrower: Address) -> ContractCall<M, U256> {
            self.0
                .method_hash([26, 122, 78, 180], (fy_token, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_transferAdmin` (0xe6abb5ae) function"]
        pub fn transfer_admin(&self, new_admin: Address) -> ContractCall<M, ()> {
            self.0
                .method_hash([230, 171, 181, 174], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `freeCollateral` (0x2abaf14e) function"]
        pub fn free_collateral(&self, fy_token: Address, collateral_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([42, 186, 241, 78], (fy_token, collateral_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentCollateralizationRatio` (0xe2ad6fe0) function"]
        pub fn get_current_collateralization_ratio(
            &self,
            fy_token: Address,
            borrower: Address,
        ) -> ContractCall<M, U256> {
            self.0
                .method_hash([226, 173, 111, 224], (fy_token, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fintroller` (0xede4c0cc) function"]
        pub fn fintroller(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([237, 228, 192, 204], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositCollateral` (0xa5d5db0c) function"]
        pub fn deposit_collateral(&self, fy_token: Address, collateral_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([165, 213, 219, 12], (fy_token, collateral_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `clutchCollateral` (0x0e496ef3) function"]
        pub fn clutch_collateral(
            &self,
            fy_token: Address,
            liquidator: Address,
            borrower: Address,
            collateral_amount: U256,
        ) -> ContractCall<M, bool> {
            self.0
                .method_hash([14, 73, 110, 243], (fy_token, liquidator, borrower, collateral_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVaultDebt` (0x8a0695fd) function"]
        pub fn get_vault_debt(&self, fy_token: Address, borrower: Address) -> ContractCall<M, U256> {
            self.0
                .method_hash([138, 6, 149, 253], (fy_token, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isBalanceSheet` (0x6321e20c) function"]
        pub fn is_balance_sheet(&self) -> ContractCall<M, bool> {
            self.0
                .method_hash([99, 33, 226, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVaultDebt` (0xb9d2cc30) function"]
        pub fn set_vault_debt(
            &self,
            fy_token: Address,
            borrower: Address,
            new_vault_debt: U256,
        ) -> ContractCall<M, bool> {
            self.0
                .method_hash([185, 210, 204, 48], (fy_token, borrower, new_vault_debt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHypotheticalCollateralizationRatio` (0xd8e51dfb) function"]
        pub fn get_hypothetical_collateralization_ratio(
            &self,
            fy_token: Address,
            borrower: Address,
            locked_collateral: U256,
            debt: U256,
        ) -> ContractCall<M, U256> {
            self.0
                .method_hash([216, 229, 29, 251], (fy_token, borrower, locked_collateral, debt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `DepositCollateral` event"]
        pub fn deposit_collateral_filter(&self) -> Event<M, DepositCollateralFilter> {
            self.0
                .event("DepositCollateral")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ClutchCollateral` event"]
        pub fn clutch_collateral_filter(&self) -> Event<M, ClutchCollateralFilter> {
            self.0
                .event("ClutchCollateral")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OpenVault` event"]
        pub fn open_vault_filter(&self) -> Event<M, OpenVaultFilter> {
            self.0
                .event("OpenVault")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetVaultDebt` event"]
        pub fn set_vault_debt_filter(&self) -> Event<M, SetVaultDebtFilter> {
            self.0
                .event("SetVaultDebt")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `WithdrawCollateral` event"]
        pub fn withdraw_collateral_filter(&self) -> Event<M, WithdrawCollateralFilter> {
            self.0
                .event("WithdrawCollateral")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `LockCollateral` event"]
        pub fn lock_collateral_filter(&self) -> Event<M, LockCollateralFilter> {
            self.0
                .event("LockCollateral")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `TransferAdmin` event"]
        pub fn transfer_admin_filter(&self) -> Event<M, TransferAdminFilter> {
            self.0
                .event("TransferAdmin")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `FreeCollateral` event"]
        pub fn free_collateral_filter(&self) -> Event<M, FreeCollateralFilter> {
            self.0
                .event("FreeCollateral")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct DepositCollateralFilter {
        pub fy_token: Address,
        pub borrower: Address,
        pub collateral_amount: U256,
    }
    impl DepositCollateralFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                239, 18, 241, 142, 43, 101, 120, 185, 27, 60, 133, 44, 66, 60, 168, 238, 83, 15, 101, 242, 15, 119, 14,
                98, 167, 206, 138, 160, 142, 26, 183, 119,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`DepositCollateral(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "DepositCollateral(address,address,uint256)"
        }
    }
    impl Detokenize for DepositCollateralFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 3 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    3,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let collateral_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(DepositCollateralFilter {
                fy_token,
                borrower,
                collateral_amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct ClutchCollateralFilter {
        pub fy_token: Address,
        pub liquidator: Address,
        pub borrower: Address,
        pub clutched_collateral_amount: U256,
    }
    impl ClutchCollateralFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                1, 215, 109, 150, 208, 113, 4, 72, 94, 25, 188, 206, 148, 99, 187, 5, 26, 70, 234, 165, 216, 204, 224,
                67, 161, 169, 203, 174, 178, 167, 34, 142,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`ClutchCollateral(address,address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "ClutchCollateral(address,address,address,uint256)"
        }
    }
    impl Detokenize for ClutchCollateralFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 4 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    4,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let liquidator = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let clutched_collateral_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(ClutchCollateralFilter {
                fy_token,
                liquidator,
                borrower,
                clutched_collateral_amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct OpenVaultFilter {
        pub fy_token: Address,
        pub borrower: Address,
    }
    impl OpenVaultFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                190, 54, 254, 170, 73, 209, 246, 107, 36, 157, 177, 209, 18, 164, 125, 50, 7, 61, 28, 228, 132, 2, 2,
                118, 215, 55, 144, 202, 208, 43, 232, 144,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`OpenVault(address,address)`"]
        pub const fn abi_signature() -> &'static str {
            "OpenVault(address,address)"
        }
    }
    impl Detokenize for OpenVaultFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 2 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    2,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(OpenVaultFilter { fy_token, borrower })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetVaultDebtFilter {
        pub fy_token: Address,
        pub borrower: Address,
        pub old_debt: U256,
        pub new_debt: U256,
    }
    impl SetVaultDebtFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                242, 210, 124, 143, 14, 135, 44, 110, 212, 41, 197, 210, 82, 120, 174, 96, 35, 174, 156, 197, 247, 136,
                160, 253, 57, 182, 111, 183, 56, 248, 244, 92,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetVaultDebt(address,address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "SetVaultDebt(address,address,uint256,uint256)"
        }
    }
    impl Detokenize for SetVaultDebtFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 4 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    4,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let old_debt = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_debt = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetVaultDebtFilter {
                fy_token,
                borrower,
                old_debt,
                new_debt,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct WithdrawCollateralFilter {
        pub fy_token: Address,
        pub borrower: Address,
        pub collateral_amount: U256,
    }
    impl WithdrawCollateralFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                22, 7, 218, 142, 145, 68, 3, 93, 133, 55, 148, 20, 37, 116, 30, 158, 53, 105, 200, 29, 52, 167, 248,
                224, 197, 196, 70, 53, 220, 113, 105, 33,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`WithdrawCollateral(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "WithdrawCollateral(address,address,uint256)"
        }
    }
    impl Detokenize for WithdrawCollateralFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 3 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    3,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let collateral_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(WithdrawCollateralFilter {
                fy_token,
                borrower,
                collateral_amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct LockCollateralFilter {
        pub fy_token: Address,
        pub borrower: Address,
        pub collateral_amount: U256,
    }
    impl LockCollateralFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                93, 236, 229, 45, 110, 70, 78, 219, 129, 165, 108, 15, 241, 111, 109, 180, 131, 120, 105, 2, 134, 146,
                13, 109, 179, 157, 56, 112, 0, 6, 77, 230,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`LockCollateral(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "LockCollateral(address,address,uint256)"
        }
    }
    impl Detokenize for LockCollateralFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 3 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    3,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let collateral_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(LockCollateralFilter {
                fy_token,
                borrower,
                collateral_amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct TransferAdminFilter {
        pub old_admin: Address,
        pub new_admin: Address,
    }
    impl TransferAdminFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                189, 211, 97, 67, 238, 9, 222, 96, 189, 239, 202, 112, 104, 14, 15, 113, 24, 155, 46, 215, 172, 238,
                54, 75, 83, 145, 122, 212, 51, 253, 175, 128,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`TransferAdmin(address,address)`"]
        pub const fn abi_signature() -> &'static str {
            "TransferAdmin(address,address)"
        }
    }
    impl Detokenize for TransferAdminFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 2 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    2,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let old_admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(TransferAdminFilter { old_admin, new_admin })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct FreeCollateralFilter {
        pub fy_token: Address,
        pub borrower: Address,
        pub collateral_amount: U256,
    }
    impl FreeCollateralFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                30, 184, 20, 210, 147, 99, 186, 13, 73, 110, 78, 235, 130, 29, 111, 37, 55, 49, 11, 131, 179, 108, 112,
                251, 151, 174, 20, 27, 65, 250, 115, 60,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`FreeCollateral(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "FreeCollateral(address,address,uint256)"
        }
    }
    impl Detokenize for FreeCollateralFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 3 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    3,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let collateral_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(FreeCollateralFilter {
                fy_token,
                borrower,
                collateral_amount,
            })
        }
    }
}
