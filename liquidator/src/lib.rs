use ethers::prelude::ContractError;
use std::result::Result;

pub mod escalator;
pub mod liquidations;
pub mod sentinel;
pub mod vaults;

pub type EthersResult<T, M> = Result<T, ContractError<M>>;
