pub mod liquidator;
pub mod sentinel;
pub mod vault;

use ethers::prelude::*;

pub type EthersResult<T, M> = std::result::Result<T, ContractError<M>>;
