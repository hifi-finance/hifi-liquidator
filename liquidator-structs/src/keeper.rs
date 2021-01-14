/// The keeper monitors the chain for underwater accounts on Hifi and sends a liquidation
/// transaction when it discovers one.
#[allow(unused)]
pub struct Keeper {
    last_block: u64,
}
