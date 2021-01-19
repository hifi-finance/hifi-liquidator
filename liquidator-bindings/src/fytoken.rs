pub use fytoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod fytoken_mod {
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
    #[doc = "FyToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FYTOKEN_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"name_\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"symbol_\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"expirationTime_\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"contract FintrollerInterface\",\n        \"name\": \"fintroller_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract BalanceSheetInterface\",\n        \"name\": \"balanceSheet_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract Erc20Interface\",\n        \"name\": \"underlying_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract Erc20Interface\",\n        \"name\": \"collateral_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Approval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Borrow\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"holder\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"burnAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Burn\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"liquidator\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"repayAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"clutchedCollateralAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LiquidateBorrow\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"beneficiary\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"mintAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Mint\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"contract Erc20Interface\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"recoverAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Recover\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"payer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"repayAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newDebt\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"RepayBorrow\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"contract FintrollerInterface\",\n        \"name\": \"oldFintroller\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"contract FintrollerInterface\",\n        \"name\": \"newFintroller\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"SetFintroller\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"contract Erc20Interface[]\",\n        \"name\": \"nonRecoverableTokens\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"SetNonRecoverableTokens\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Transfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"oldAdmin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"TransferAdmin\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"DOMAIN_SEPARATOR\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"PERMIT_TYPEHASH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract Erc20Interface\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"recoverAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"_recover\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"_renounceAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract FintrollerInterface\",\n        \"name\": \"newFintroller\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"_setFintroller\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract Erc20Interface[]\",\n        \"name\": \"tokens\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"_setNonRecoverableTokens\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"_transferAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"admin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"balanceOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"balanceSheet\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract BalanceSheetInterface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"borrow\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"holder\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"burnAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"burn\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"collateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract Erc20Interface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"collateralPrecisionScalar\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"subtractedValue\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"decreaseAllowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"expirationTime\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"fintroller\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract FintrollerInterface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"addedValue\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"increaseAllowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"isFyToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"isMatured\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"repayAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"liquidateBorrow\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"beneficiary\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"mintAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"mint\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"nonRecoverableTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract Erc20Interface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"nonces\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"r\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"s\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"permit\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"redemptionPool\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract RedemptionPoolInterface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"repayAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"repayBorrow\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"repayAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"repayBorrowBehalf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"symbol\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalSupply\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transfer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transferFrom\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"underlying\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract Erc20Interface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"underlyingPrecisionScalar\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"version\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct FyToken<M>(Contract<M>);
    impl<M> std::ops::Deref for FyToken<M> {
        type Target = Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: Middleware> std::fmt::Debug for FyToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FyToken)).field(&self.address()).finish()
        }
    }
    impl<'a, M: Middleware> FyToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>>(address: T, client: Arc<M>) -> Self {
            let contract = Contract::new(address.into(), FYTOKEN_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `nonRecoverableTokens` (0x9e6ae5a0) function"]
        pub fn non_recoverable_tokens(&self, p0: U256) -> ContractCall<M, Address> {
            self.0
                .method_hash([158, 106, 229, 160], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fintroller` (0xede4c0cc) function"]
        pub fn fintroller(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([237, 228, 192, 204], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_transferAdmin` (0xe6abb5ae) function"]
        pub fn transfer_admin(&self, new_admin: Address) -> ContractCall<M, ()> {
            self.0
                .method_hash([230, 171, 181, 174], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(&self, spender: Address, added_value: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x9dc29fac) function"]
        pub fn burn(&self, holder: Address, burn_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([157, 194, 159, 172], (holder, burn_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setNonRecoverableTokens` (0xdde4bec0) function"]
        pub fn set_non_recoverable_tokens(&self, tokens: Vec<Address>) -> ContractCall<M, ()> {
            self.0
                .method_hash([221, 228, 190, 192], tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isFyToken` (0xcab364f1) function"]
        pub fn is_fy_token(&self) -> ContractCall<M, bool> {
            self.0
                .method_hash([202, 179, 100, 241], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceSheet` (0x22285cf6) function"]
        pub fn balance_sheet(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([34, 40, 92, 246], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collateral` (0xd8dfeb45) function"]
        pub fn collateral(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([216, 223, 235, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(&self, recipient: Address, amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redemptionPool` (0x7c4950c7) function"]
        pub fn redemption_pool(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([124, 73, 80, 199], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collateralPrecisionScalar` (0xb2bc5ef9) function"]
        pub fn collateral_precision_scalar(&self) -> ContractCall<M, U256> {
            self.0
                .method_hash([178, 188, 94, 249], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(&self) -> ContractCall<M, U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(&self, spender: Address, subtracted_value: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBorrow` (0x0e752702) function"]
        pub fn repay_borrow(&self, repay_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([14, 117, 39, 2], repay_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlyingPrecisionScalar` (0x1a0de6fa) function"]
        pub fn underlying_precision_scalar(&self) -> ContractCall<M, U256> {
            self.0
                .method_hash([26, 13, 230, 250], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(&self, spender: Address, amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: Address,
            spender: Address,
            amount: U256,
            deadline: U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ContractCall<M, ()> {
            self.0
                .method_hash([213, 5, 172, 207], (owner, spender, amount, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrow` (0xc5ebeaec) function"]
        pub fn borrow(&self, borrow_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([197, 235, 234, 236], borrow_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateBorrow` (0xe6e0d75c) function"]
        pub fn liquidate_borrow(&self, borrower: Address, repay_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([230, 224, 215, 92], (borrower, repay_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expirationTime` (0xda284dcc) function"]
        pub fn expiration_time(&self) -> ContractCall<M, U256> {
            self.0
                .method_hash([218, 40, 77, 204], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isMatured` (0x7f2b6a0d) function"]
        pub fn is_matured(&self) -> ContractCall<M, bool> {
            self.0
                .method_hash([127, 43, 106, 13], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(&self, beneficiary: Address, mint_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([64, 193, 15, 25], (beneficiary, mint_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBorrowBehalf` (0x2608f818) function"]
        pub fn repay_borrow_behalf(&self, borrower: Address, repay_amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([38, 8, 248, 24], (borrower, repay_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(&self, p0: Address) -> ContractCall<M, U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(&self, sender: Address, recipient: Address, amount: U256) -> ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(&self, owner: Address, spender: Address) -> ContractCall<M, U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_recover` (0x1c2872aa) function"]
        pub fn recover(&self, token: Address, recover_amount: U256) -> ContractCall<M, ()> {
            self.0
                .method_hash([28, 40, 114, 170], (token, recover_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(&self, account: Address) -> ContractCall<M, U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_renounceAdmin` (0xbfa25308) function"]
        pub fn renounce_admin(&self) -> ContractCall<M, ()> {
            self.0
                .method_hash([191, 162, 83, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlying` (0x6f307dc3) function"]
        pub fn underlying(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(&self) -> ContractCall<M, String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setFintroller` (0x1eabf00d) function"]
        pub fn set_fintroller(&self, new_fintroller: Address) -> ContractCall<M, bool> {
            self.0
                .method_hash([30, 171, 240, 13], new_fintroller)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> Event<M, TransferFilter> {
            self.0
                .event("Transfer")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `RepayBorrow` event"]
        pub fn repay_borrow_filter(&self) -> Event<M, RepayBorrowFilter> {
            self.0
                .event("RepayBorrow")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `TransferAdmin` event"]
        pub fn transfer_admin_filter(&self) -> Event<M, TransferAdminFilter> {
            self.0
                .event("TransferAdmin")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Borrow` event"]
        pub fn borrow_filter(&self) -> Event<M, BorrowFilter> {
            self.0
                .event("Borrow")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> Event<M, ApprovalFilter> {
            self.0
                .event("Approval")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> Event<M, MintFilter> {
            self.0
                .event("Mint")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> Event<M, BurnFilter> {
            self.0
                .event("Burn")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `LiquidateBorrow` event"]
        pub fn liquidate_borrow_filter(&self) -> Event<M, LiquidateBorrowFilter> {
            self.0
                .event("LiquidateBorrow")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Recover` event"]
        pub fn recover_filter(&self) -> Event<M, RecoverFilter> {
            self.0
                .event("Recover")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetFintroller` event"]
        pub fn set_fintroller_filter(&self) -> Event<M, SetFintrollerFilter> {
            self.0
                .event("SetFintroller")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `SetNonRecoverableTokens` event"]
        pub fn set_non_recoverable_tokens_filter(&self) -> Event<M, SetNonRecoverableTokensFilter> {
            self.0
                .event("SetNonRecoverableTokens")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct TransferFilter {
        pub from: Address,
        pub to: Address,
        pub amount: U256,
    }
    impl TransferFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                221, 242, 82, 173, 27, 226, 200, 155, 105, 194, 176, 104, 252, 55, 141, 170, 149, 43, 167, 241, 99,
                196, 161, 22, 40, 245, 90, 77, 245, 35, 179, 239,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Transfer(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Transfer(address,address,uint256)"
        }
    }
    impl Detokenize for TransferFilter {
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
            let from = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let to = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(TransferFilter { from, to, amount })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct RepayBorrowFilter {
        pub payer: Address,
        pub borrower: Address,
        pub repay_amount: U256,
        pub new_debt: U256,
    }
    impl RepayBorrowFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                169, 161, 84, 35, 122, 105, 146, 47, 136, 96, 50, 29, 31, 236, 22, 36, 165, 219, 232, 168, 175, 137,
                163, 221, 61, 122, 117, 159, 108, 128, 128, 216,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`RepayBorrow(address,address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "RepayBorrow(address,address,uint256,uint256)"
        }
    }
    impl Detokenize for RepayBorrowFilter {
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
            let payer = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let repay_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_debt = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(RepayBorrowFilter {
                payer,
                borrower,
                repay_amount,
                new_debt,
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
    pub struct BorrowFilter {
        pub borrower: Address,
        pub borrow_amount: U256,
    }
    impl BorrowFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                203, 192, 78, 202, 126, 157, 163, 92, 177, 57, 58, 97, 53, 161, 153, 202, 82, 228, 80, 213, 233, 37,
                28, 189, 153, 247, 132, 125, 51, 163, 103, 80,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Borrow(address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Borrow(address,uint256)"
        }
    }
    impl Detokenize for BorrowFilter {
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
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrow_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(BorrowFilter {
                borrower,
                borrow_amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct ApprovalFilter {
        pub owner: Address,
        pub spender: Address,
        pub amount: U256,
    }
    impl ApprovalFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                140, 91, 225, 229, 235, 236, 125, 91, 209, 79, 113, 66, 125, 30, 132, 243, 221, 3, 20, 192, 247, 178,
                41, 30, 91, 32, 10, 200, 199, 195, 185, 37,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Approval(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Approval(address,address,uint256)"
        }
    }
    impl Detokenize for ApprovalFilter {
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
            let owner = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let spender = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(ApprovalFilter { owner, spender, amount })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct MintFilter {
        pub beneficiary: Address,
        pub mint_amount: U256,
    }
    impl MintFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                15, 103, 152, 165, 96, 121, 58, 84, 195, 188, 254, 134, 169, 60, 222, 30, 115, 8, 125, 148, 76, 14,
                162, 5, 68, 19, 125, 65, 33, 57, 104, 133,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Mint(address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Mint(address,uint256)"
        }
    }
    impl Detokenize for MintFilter {
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
            let beneficiary = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let mint_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(MintFilter {
                beneficiary,
                mint_amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct BurnFilter {
        pub holder: Address,
        pub burn_amount: U256,
    }
    impl BurnFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                204, 22, 245, 219, 180, 135, 50, 128, 129, 92, 30, 224, 157, 189, 6, 115, 108, 255, 204, 24, 68, 18,
                207, 122, 113, 160, 253, 183, 93, 57, 124, 165,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Burn(address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Burn(address,uint256)"
        }
    }
    impl Detokenize for BurnFilter {
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
            let holder = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let burn_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(BurnFilter { holder, burn_amount })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct LiquidateBorrowFilter {
        pub liquidator: Address,
        pub borrower: Address,
        pub repay_amount: U256,
        pub clutched_collateral_amount: U256,
    }
    impl LiquidateBorrowFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                8, 177, 151, 182, 174, 102, 209, 142, 230, 250, 32, 168, 38, 168, 69, 121, 169, 37, 185, 230, 70, 163,
                191, 25, 233, 111, 89, 237, 86, 140, 38, 136,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`LiquidateBorrow(address,address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "LiquidateBorrow(address,address,uint256,uint256)"
        }
    }
    impl Detokenize for LiquidateBorrowFilter {
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
            let liquidator = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let borrower = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let repay_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let clutched_collateral_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(LiquidateBorrowFilter {
                liquidator,
                borrower,
                repay_amount,
                clutched_collateral_amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct RecoverFilter {
        pub admin: Address,
        pub token: Address,
        pub recover_amount: U256,
    }
    impl RecoverFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                246, 137, 28, 132, 166, 198, 175, 50, 166, 208, 82, 23, 42, 138, 204, 76, 99, 27, 29, 80, 87, 255, 162,
                188, 29, 162, 104, 182, 147, 142, 162, 218,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Recover(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Recover(address,address,uint256)"
        }
    }
    impl Detokenize for RecoverFilter {
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
            let token = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let recover_amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(RecoverFilter {
                admin,
                token,
                recover_amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetFintrollerFilter {
        pub admin: Address,
        pub old_fintroller: Address,
        pub new_fintroller: Address,
    }
    impl SetFintrollerFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                5, 245, 239, 240, 120, 169, 76, 2, 99, 119, 171, 108, 107, 220, 109, 42, 209, 147, 42, 62, 73, 113,
                253, 116, 115, 52, 138, 243, 96, 158, 197, 122,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetFintroller(address,address,address)`"]
        pub const fn abi_signature() -> &'static str {
            "SetFintroller(address,address,address)"
        }
    }
    impl Detokenize for SetFintrollerFilter {
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
            let old_fintroller = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_fintroller = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetFintrollerFilter {
                admin,
                old_fintroller,
                new_fintroller,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct SetNonRecoverableTokensFilter {
        pub admin: Address,
        pub non_recoverable_tokens: Vec<Address>,
    }
    impl SetNonRecoverableTokensFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                216, 119, 84, 59, 239, 39, 218, 144, 54, 36, 52, 33, 230, 35, 65, 60, 143, 221, 120, 215, 209, 131,
                204, 147, 216, 243, 205, 219, 214, 87, 162, 237,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`SetNonRecoverableTokens(address,address[])`"]
        pub const fn abi_signature() -> &'static str {
            "SetNonRecoverableTokens(address,address[])"
        }
    }
    impl Detokenize for SetNonRecoverableTokensFilter {
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
            let non_recoverable_tokens = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(SetNonRecoverableTokensFilter {
                admin,
                non_recoverable_tokens,
            })
        }
    }
}
