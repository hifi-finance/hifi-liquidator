use std::path::PathBuf;

use ethers::prelude::{Address, U256};
use gumdrop::Options;
use serde::Deserialize;

// CLI Options
#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(help = "Path to json file with the contract addresses")]
    config: PathBuf,

    #[options(help = "File to be used for persistence", default = "data.json")]
    file: PathBuf,

    #[options(help = "Polling interval in milliseconds", default = "1000")]
    interval: u64,

    #[options(help = "Minimum desired profit per liquidation", default = "0")]
    min_profit: U256,

    #[options(help = "Path to your private key")]
    private_key: PathBuf,

    #[options(help = "Block at which to begin monitoring")]
    start_block: Option<u64>,

    #[options(help = "Ethereum node endpoint (HTTP or WS)", default = "http://localhost:8545")]
    url: String,
}

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "BalanceSheet")]
    balance_sheet: Address,
    #[serde(rename = "Fintroller")]
    fintroller: Address,
    #[serde(rename = "FlashLiquidate")]
    flash_liquidate: Address,
    #[serde(rename = "Multicall")]
    multicall: Option<Address>,
    #[serde(rename = "UniswapPair")]
    uniswap_pair: Address,
}
