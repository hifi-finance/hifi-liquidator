pub mod escalator;
pub mod liquidator;
pub mod vault;

use ethers::prelude::*;

pub type HifiResult<T, M> = std::result::Result<T, ContractError<M>>;
