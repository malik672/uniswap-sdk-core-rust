use crate::prelude::*;
use alloy_primitives::U256;
use num_bigint::Sign;

/// Represents the various types of trades.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TradeType {
    /// Indicates that the trade is based on an exact input amount.
    ExactInput,

    /// Indicates that the trade is based on an exact output amount.
    ExactOutput,
}

/// Represents three various ways to round
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Rounding {
    /// Rounds down to the nearest whole number.
    RoundDown,

    /// Rounds to the nearest whole number, rounding halfway cases away from zero.
    RoundHalfUp,

    /// Rounds up to the nearest whole number.
    RoundUp,
}

lazy_static! {
    /// Represents the maximum amount contained in a uint256
    pub static ref MAX_UINT256: BigInt =
        BigInt::from_biguint(Sign::Plus, BigUint::from_bytes_le(&U256::MAX.as_le_bytes()));
}
