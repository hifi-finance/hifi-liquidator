use ethers::prelude::ContractError;
use std::result::Result;

pub mod liquidator;
pub mod sentinel;
pub mod vault;

pub type EthersResult<T, M> = Result<T, ContractError<M>>;
