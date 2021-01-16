use ethers::prelude::Address;
use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "BalanceSheet")]
    pub balance_sheet: Address,

    #[serde(rename = "FyTokens")]
    pub fy_tokens: Vec<Address>,

    #[serde(rename = "HifiFlashSwap")]
    pub hifi_flash_swap: Address,

    #[serde(rename = "Multicall")]
    pub multicall: Option<Address>,

    #[serde(rename = "UniswapV2Pair")]
    pub uniswap_v2_pair: Address,
}
