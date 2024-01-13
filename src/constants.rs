use crate::prelude::*;
use alloy_primitives::U256;
use num_bigint::Sign;

pub enum TradeType {
    ExactInput,
    ExactOutput,
}

pub enum Rounding {
    RoundDown,
    RoundHalfUp,
    RoundUp,
}

lazy_static! {
    pub static ref MAX_UINT256: BigInt =
        BigInt::from_bytes_be(Sign::Plus, &U256::MAX.to_be_bytes::<32>());
}
