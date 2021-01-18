use ethers::{middleware::gas_escalator::GeometricGasPrice, types::U256};

pub fn init_gas_escalator() -> GeometricGasPrice {
    let coefficient = 1.12501;
    let every_secs: u64 = 5; // TODO: Make this be 90s
    let max_gas_price = Some(U256::from(5000 * 1e9 as u64)); // 5k gwei
    GeometricGasPrice::new(coefficient, every_secs, max_gas_price)
}
