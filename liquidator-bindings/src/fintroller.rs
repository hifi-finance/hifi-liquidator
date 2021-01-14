#[allow(clippy::all)]
pub use fintroller_mod::*;
#[allow(clippy::too_many_arguments)]
mod fintroller_mod {
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
    #[doc = "Fintroller was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FINTROLLER_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ListBond\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"oldCollateralizationRatio\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newCollateralizationRatio\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SetBondCollateralizationRatio\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"oldDebtCeiling\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newDebtCeiling\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SetBondDebtCeiling\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"SetBorrowAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"SetDepositCollateralAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"SetLiquidateBorrowAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"oldLiquidationIncentive\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newLiquidationIncentive\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SetLiquidationIncentive\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"oldOracle\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"newOracle\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"SetOracle\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"SetRedeemFyTokensAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"SetRepayBorrowAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"SetSupplyUnderlyingAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"oldAdmin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"TransferAdmin\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"_renounceAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"_transferAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"admin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getBond\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralizationRatioMantissa\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"debtCeiling\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isBorrowAllowed\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isDepositCollateralAllowed\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isLiquidateBorrowAllowed\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isListed\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isRedeemFyTokenAllowed\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isRepayBorrowAllowed\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isSupplyUnderlyingAllowed\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getBondCollateralizationRatio\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getBondDebtCeiling\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getBorrowAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getDepositCollateralAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getLiquidateBorrowAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getRedeemFyTokensAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getRepayBorrowAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getSupplyUnderlyingAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"isFintroller\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"liquidationIncentiveMantissa\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"listBond\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"oracle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract UniswapAnchoredViewInterface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"oraclePricePrecisionScalar\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newCollateralizationRatioMantissa\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setBondCollateralizationRatio\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newDebtCeiling\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setBondDebtCeiling\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setBorrowAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setDepositCollateralAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setLiquidateBorrowAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newLiquidationIncentiveMantissa\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setLiquidationIncentive\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract UniswapAnchoredViewInterface\",\n        \"name\": \"newOracle\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setOracle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setRedeemFyTokensAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setRepayBorrowAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FyTokenInterface\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setSupplyUnderlyingAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct Fintroller<M>(Contract<M>);
    impl<M> std::ops::Deref for Fintroller<M> {
        type Target = Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: Middleware> std::fmt::Debug for Fintroller<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Fintroller)).field(&self.address()).finish()
        }
    }
    impl<'a, M: Middleware> Fintroller<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>>(address: T, client: Arc<M>) -> Self {
            let contract = Contract::new(address.into(), FINTROLLER_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `_transferAdmin` (0xe6abb5ae) function"]
        pub fn transfer_admin(&self, new_admin: Address) -> ContractCall<M, ()> {
            self.0
                .method_hash([230, 171, 181, 174], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLiquidateBorrowAllowed` (0xbb23ffec) function"]
        pub fn get_liquidate_borrow_allowed(&self, fy_token: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([187, 35, 255, 236], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDepositCollateralAllowed` (0xce8f6d3e) function"]
        pub fn get_deposit_collateral_allowed(&self, fy_token: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([206, 143, 109, 62], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBorrowAllowed` (0x227661cb) function"]
        pub fn set_borrow_allowed(&self, fy_token: Address, state: bool) -> ContractCall<M, bool> {
            self.0
                .method_hash([34, 118, 97, 203], (fy_token, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationIncentiveMantissa` (0x4ada90af) function"]
        pub fn liquidation_incentive_mantissa(&self) -> ContractCall<M, U256> {
            self.0
                .method_hash([74, 218, 144, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidationIncentive` (0xa8431081) function"]
        pub fn set_liquidation_incentive(&self, new_liquidation_incentive_mantissa: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([168, 67, 16, 129], new_liquidation_incentive_mantissa)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRedeemFyTokensAllowed` (0xa0642c9c) function"]
        pub fn set_redeem_fy_tokens_allowed(&self, fy_token: Address, state: bool) -> ContractCall<M, bool> {
            self.0
                .method_hash([160, 100, 44, 156], (fy_token, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBondDebtCeiling` (0xd010b00f) function"]
        pub fn get_bond_debt_ceiling(&self, fy_token: Address) -> ContractCall<M, U256> {
            self.0
                .method_hash([208, 16, 176, 15], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupplyUnderlyingAllowed` (0xe79628ab) function"]
        pub fn get_supply_underlying_allowed(&self, fy_token: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([231, 150, 40, 171], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRepayBorrowAllowed` (0xd59f3f53) function"]
        pub fn set_repay_borrow_allowed(&self, fy_token: Address, state: bool) -> ContractCall<M, bool> {
            self.0
                .method_hash([213, 159, 63, 83], (fy_token, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isFintroller` (0x58f25c50) function"]
        pub fn is_fintroller(&self) -> ContractCall<M, bool> {
            self.0
                .method_hash([88, 242, 92, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDepositCollateralAllowed` (0x7922911f) function"]
        pub fn set_deposit_collateral_allowed(&self, fy_token: Address, state: bool) -> ContractCall<M, bool> {
            self.0
                .method_hash([121, 34, 145, 31], (fy_token, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSupplyUnderlyingAllowed` (0x6d41a27c) function"]
        pub fn set_supply_underlying_allowed(&self, fy_token: Address, state: bool) -> ContractCall<M, bool> {
            self.0
                .method_hash([109, 65, 162, 124], (fy_token, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBorrowAllowed` (0x81a7bc97) function"]
        pub fn get_borrow_allowed(&self, fy_token: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([129, 167, 188, 151], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBondCollateralizationRatio` (0xd4769104) function"]
        pub fn get_bond_collateralization_ratio(&self, fy_token: Address) -> ContractCall<M, U256> {
            self.0
                .method_hash([212, 118, 145, 4], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRepayBorrowAllowed` (0xe60f0773) function"]
        pub fn get_repay_borrow_allowed(&self, fy_token: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([230, 15, 7, 115], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBondDebtCeiling` (0x931c759e) function"]
        pub fn set_bond_debt_ceiling(&self, fy_token: Address, new_debt_ceiling: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([147, 28, 117, 158], (fy_token, new_debt_ceiling))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRedeemFyTokensAllowed` (0x162fdba3) function"]
        pub fn get_redeem_fy_tokens_allowed(&self, fy_token: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([22, 47, 219, 163], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidateBorrowAllowed` (0x02b5bda7) function"]
        pub fn set_liquidate_borrow_allowed(&self, fy_token: Address, state: bool) -> ContractCall<M, bool> {
            self.0
                .method_hash([2, 181, 189, 167], (fy_token, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOracle` (0x7adbf973) function"]
        pub fn set_oracle(&self, new_oracle: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([122, 219, 249, 115], new_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oraclePricePrecisionScalar` (0xfa23f21a) function"]
        pub fn oracle_price_precision_scalar(&self) -> ContractCall<M, U256> {
            self.0
                .method_hash([250, 35, 242, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `listBond` (0x8559d20d) function"]
        pub fn list_bond(&self, fy_token: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([133, 89, 210, 13], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_renounceAdmin` (0xbfa25308) function"]
        pub fn renounce_admin(&self) -> ContractCall<M, ()> {
            self.0
                .method_hash([191, 162, 83, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBond` (0x0d8912f3) function"]
        pub fn get_bond(
            &self,
            fy_token: Address,
        ) -> ContractCall<M, (U256, U256, bool, bool, bool, bool, bool, bool, bool)> {
            self.0
                .method_hash([13, 137, 18, 243], fy_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBondCollateralizationRatio` (0x358e8216) function"]
        pub fn set_bond_collateralization_ratio(
            &self,
            fy_token: Address,
            new_collateralization_ratio_mantissa: U256,
        ) -> ContractCall<M, bool> {
            self.0
                .method_hash([53, 142, 130, 22], (fy_token, new_collateralization_ratio_mantissa))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetSupplyUnderlyingAllowed` event"]
        pub fn set_supply_underlying_allowed_filter(&self) -> Event<M, SetSupplyUnderlyingAllowedFilter> {
            self.0
                .event("SetSupplyUnderlyingAllowed")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetDepositCollateralAllowed` event"]
        pub fn set_deposit_collateral_allowed_filter(&self) -> Event<M, SetDepositCollateralAllowedFilter> {
            self.0
                .event("SetDepositCollateralAllowed")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetRedeemFyTokensAllowed` event"]
        pub fn set_redeem_fy_tokens_allowed_filter(&self) -> Event<M, SetRedeemFyTokensAllowedFilter> {
            self.0
                .event("SetRedeemFyTokensAllowed")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetRepayBorrowAllowed` event"]
        pub fn set_repay_borrow_allowed_filter(&self) -> Event<M, SetRepayBorrowAllowedFilter> {
            self.0
                .event("SetRepayBorrowAllowed")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetLiquidationIncentive` event"]
        pub fn set_liquidation_incentive_filter(&self) -> Event<M, SetLiquidationIncentiveFilter> {
            self.0
                .event("SetLiquidationIncentive")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetBorrowAllowed` event"]
        pub fn set_borrow_allowed_filter(&self) -> Event<M, SetBorrowAllowedFilter> {
            self.0
                .event("SetBorrowAllowed")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetBondDebtCeiling` event"]
        pub fn set_bond_debt_ceiling_filter(&self) -> Event<M, SetBondDebtCeilingFilter> {
            self.0
                .event("SetBondDebtCeiling")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetOracle` event"]
        pub fn set_oracle_filter(&self) -> Event<M, SetOracleFilter> {
            self.0
                .event("SetOracle")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetBondCollateralizationRatio` event"]
        pub fn set_bond_collateralization_ratio_filter(&self) -> Event<M, SetBondCollateralizationRatioFilter> {
            self.0
                .event("SetBondCollateralizationRatio")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetLiquidateBorrowAllowed` event"]
        pub fn set_liquidate_borrow_allowed_filter(&self) -> Event<M, SetLiquidateBorrowAllowedFilter> {
            self.0
                .event("SetLiquidateBorrowAllowed")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `TransferAdmin` event"]
        pub fn transfer_admin_filter(&self) -> Event<M, TransferAdminFilter> {
            self.0
                .event("TransferAdmin")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ListBond` event"]
        pub fn list_bond_filter(&self) -> Event<M, ListBondFilter> {
            self.0
                .event("ListBond")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetSupplyUnderlyingAllowedFilter {
        pub admin: Address,
        pub fy_token: Address,
        pub state: bool,
    }
    impl SetSupplyUnderlyingAllowedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                238, 179, 110, 16, 175, 42, 2, 202, 151, 240, 45, 84, 176, 252, 236, 124, 26, 244, 232, 245, 142, 31,
                142, 38, 50, 127, 242, 36, 91, 199, 94, 95,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetSupplyUnderlyingAllowed(address,address,bool)`"]
        pub const fn abi_signature() -> &'static str {
            "SetSupplyUnderlyingAllowed(address,address,bool)"
        }
    }
    impl Detokenize for SetSupplyUnderlyingAllowedFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let state = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetSupplyUnderlyingAllowedFilter { admin, fy_token, state })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetDepositCollateralAllowedFilter {
        pub admin: Address,
        pub fy_token: Address,
        pub state: bool,
    }
    impl SetDepositCollateralAllowedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                15, 59, 144, 113, 41, 123, 96, 57, 62, 153, 6, 23, 12, 30, 34, 98, 201, 244, 158, 72, 104, 52, 99, 194,
                38, 142, 107, 34, 20, 160, 44, 130,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetDepositCollateralAllowed(address,address,bool)`"]
        pub const fn abi_signature() -> &'static str {
            "SetDepositCollateralAllowed(address,address,bool)"
        }
    }
    impl Detokenize for SetDepositCollateralAllowedFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let state = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetDepositCollateralAllowedFilter { admin, fy_token, state })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetRedeemFyTokensAllowedFilter {
        pub admin: Address,
        pub fy_token: Address,
        pub state: bool,
    }
    impl SetRedeemFyTokensAllowedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                164, 199, 229, 138, 49, 101, 185, 180, 4, 231, 140, 162, 100, 31, 252, 203, 23, 217, 193, 22, 45, 237,
                165, 136, 158, 222, 235, 187, 64, 188, 98, 210,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetRedeemFyTokensAllowed(address,address,bool)`"]
        pub const fn abi_signature() -> &'static str {
            "SetRedeemFyTokensAllowed(address,address,bool)"
        }
    }
    impl Detokenize for SetRedeemFyTokensAllowedFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let state = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetRedeemFyTokensAllowedFilter { admin, fy_token, state })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetRepayBorrowAllowedFilter {
        pub admin: Address,
        pub fy_token: Address,
        pub state: bool,
    }
    impl SetRepayBorrowAllowedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                16, 66, 77, 52, 249, 38, 212, 221, 65, 226, 249, 1, 20, 122, 183, 176, 66, 234, 198, 191, 109, 30, 134,
                32, 91, 206, 34, 195, 116, 76, 196, 38,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetRepayBorrowAllowed(address,address,bool)`"]
        pub const fn abi_signature() -> &'static str {
            "SetRepayBorrowAllowed(address,address,bool)"
        }
    }
    impl Detokenize for SetRepayBorrowAllowedFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let state = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetRepayBorrowAllowedFilter { admin, fy_token, state })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetLiquidationIncentiveFilter {
        pub admin: Address,
        pub old_liquidation_incentive: U256,
        pub new_liquidation_incentive: U256,
    }
    impl SetLiquidationIncentiveFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                51, 93, 79, 54, 11, 201, 226, 243, 90, 20, 92, 252, 103, 135, 227, 85, 69, 157, 183, 217, 216, 243, 23,
                230, 163, 131, 157, 156, 142, 246, 151, 147,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetLiquidationIncentive(address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "SetLiquidationIncentive(address,uint256,uint256)"
        }
    }
    impl Detokenize for SetLiquidationIncentiveFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let old_liquidation_incentive = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_liquidation_incentive = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetLiquidationIncentiveFilter {
                admin,
                old_liquidation_incentive,
                new_liquidation_incentive,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetBorrowAllowedFilter {
        pub admin: Address,
        pub fy_token: Address,
        pub state: bool,
    }
    impl SetBorrowAllowedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                180, 21, 202, 69, 177, 53, 227, 210, 235, 35, 37, 113, 39, 97, 152, 172, 80, 51, 7, 67, 236, 35, 231,
                116, 92, 120, 165, 183, 138, 10, 27, 81,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetBorrowAllowed(address,address,bool)`"]
        pub const fn abi_signature() -> &'static str {
            "SetBorrowAllowed(address,address,bool)"
        }
    }
    impl Detokenize for SetBorrowAllowedFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let state = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetBorrowAllowedFilter { admin, fy_token, state })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetBondDebtCeilingFilter {
        pub admin: Address,
        pub fy_token: Address,
        pub old_debt_ceiling: U256,
        pub new_debt_ceiling: U256,
    }
    impl SetBondDebtCeilingFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                206, 187, 240, 101, 208, 98, 3, 73, 234, 25, 87, 221, 45, 44, 5, 73, 191, 98, 97, 157, 60, 2, 156, 56,
                210, 120, 93, 225, 170, 72, 41, 147,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetBondDebtCeiling(address,address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "SetBondDebtCeiling(address,address,uint256,uint256)"
        }
    }
    impl Detokenize for SetBondDebtCeilingFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let old_debt_ceiling = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_debt_ceiling = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetBondDebtCeilingFilter {
                admin,
                fy_token,
                old_debt_ceiling,
                new_debt_ceiling,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetOracleFilter {
        pub admin: Address,
        pub old_oracle: Address,
        pub new_oracle: Address,
    }
    impl SetOracleFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                138, 41, 33, 146, 108, 206, 238, 201, 200, 116, 2, 5, 91, 166, 237, 182, 167, 82, 35, 73, 112, 243,
                183, 184, 249, 145, 131, 182, 34, 179, 224, 20,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetOracle(address,address,address)`"]
        pub const fn abi_signature() -> &'static str {
            "SetOracle(address,address,address)"
        }
    }
    impl Detokenize for SetOracleFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let old_oracle = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_oracle = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetOracleFilter {
                admin,
                old_oracle,
                new_oracle,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetBondCollateralizationRatioFilter {
        pub admin: Address,
        pub fy_token: Address,
        pub old_collateralization_ratio: U256,
        pub new_collateralization_ratio: U256,
    }
    impl SetBondCollateralizationRatioFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                122, 240, 38, 226, 236, 163, 55, 212, 22, 185, 176, 178, 46, 40, 161, 250, 68, 243, 120, 169, 14, 208,
                201, 109, 100, 178, 93, 136, 172, 174, 251, 42,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetBondCollateralizationRatio(address,address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "SetBondCollateralizationRatio(address,address,uint256,uint256)"
        }
    }
    impl Detokenize for SetBondCollateralizationRatioFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let old_collateralization_ratio =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_collateralization_ratio =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetBondCollateralizationRatioFilter {
                admin,
                fy_token,
                old_collateralization_ratio,
                new_collateralization_ratio,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetLiquidateBorrowAllowedFilter {
        pub admin: Address,
        pub fy_token: Address,
        pub state: bool,
    }
    impl SetLiquidateBorrowAllowedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                113, 220, 13, 53, 225, 185, 238, 23, 31, 27, 138, 201, 81, 29, 5, 228, 96, 237, 116, 22, 204, 64, 31,
                228, 211, 57, 120, 196, 79, 28, 163, 91,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetLiquidateBorrowAllowed(address,address,bool)`"]
        pub const fn abi_signature() -> &'static str {
            "SetLiquidateBorrowAllowed(address,address,bool)"
        }
    }
    impl Detokenize for SetLiquidateBorrowAllowedFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let state = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetLiquidateBorrowAllowedFilter { admin, fy_token, state })
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
    pub struct ListBondFilter {
        pub admin: Address,
        pub fy_token: Address,
    }
    impl ListBondFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                216, 27, 202, 61, 1, 238, 72, 198, 117, 163, 99, 84, 9, 160, 222, 159, 22, 93, 33, 222, 56, 209, 179,
                5, 102, 222, 43, 118, 75, 150, 205, 18,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`ListBond(address,address)`"]
        pub const fn abi_signature() -> &'static str {
            "ListBond(address,address)"
        }
    }
    impl Detokenize for ListBondFilter {
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
            let admin = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fy_token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(ListBondFilter { admin, fy_token })
        }
    }
}
