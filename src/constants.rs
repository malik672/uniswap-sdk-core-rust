use crate::prelude::*;
use alloy_primitives::U256;
use num_bigint::Sign;

/// Represents the various type of trades.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeType {
    /// Indicates that the trade is based on an exact input amount.
    ExactInput,

    /// Indicates that the trade is based on an exact output amount.
    ExactOutput,
}

#[derive(Clone, Debug, PartialEq, Copy)]

/// Represents three various way to rounds
pub enum Rounding {
    /// Rounds down to the nearest whole number.
    RoundDown,

    /// Rounds to the nearest whole number, rounding halfway cases away from zero.
    RoundHalfUp,

    /// Rounds up to the nearest whole number.
    RoundUp,
}

lazy_static! {
    ///Represnts Maximum amount contained in a uint256
    pub static ref MAX_UINT256: BigInt =
        BigInt::from_bytes_be(Sign::Plus, &U256::MAX.to_be_bytes::<32>());
}
