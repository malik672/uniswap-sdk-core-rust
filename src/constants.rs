use crate::prelude::*;
use alloy_primitives::U256;

/// Represents the various types of trades.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TradeType {
    /// Indicates that the trade is based on an exact input amount.
    ExactInput,

    /// Indicates that the trade is based on an exact output amount.
    ExactOutput,
}

/// Represents three various ways to round
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum Rounding {
    /// Rounds down to the nearest whole number.
    RoundDown,

    /// Rounds to the nearest whole number, rounding halfway cases away from zero.
    #[default]
    RoundHalfUp,

    /// Rounds up to the nearest whole number.
    RoundUp,
}

/// Represents the maximum amount contained in a uint256
pub const MAX_UINT256: BigInt =
    BigInt::from_bits(BigUint::from_le_slice(&U256::MAX.to_le_bytes::<32>()).unwrap());
