pub mod escalator;
pub mod liquidator;
pub mod sentinel;
pub mod vault;

use ethers::prelude::*;

pub type HifiLiquidatorResult<T, M> = std::result::Result<T, ContractError<M>>;
