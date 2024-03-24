use crate::prelude::*;
use alloy_primitives::U256;
use num_bigint::Sign;

#[derive(Clone, Copy, Debug, PartialEq)]
/// Represents the type of trade in the Uniswap SDK.
///
/// This enum is used to specify whether a trade is an exact input or an exact output.
pub enum TradeType {
    /// Indicates that the trade is based on an exact input amount.
    ExactInput,
    /// Indicates that the trade is based on an exact output amount.
    ExactOutput,
}

#[derive(Clone, Debug, PartialEq, Copy)]
/// Represents a rounding strategy.
pub enum Rounding {
    /// Rounds down to the nearest whole number.
    ///
    /// This variant indicates that the rounding should always go towards zero.
    RoundDown,
    /// Rounds to the nearest whole number, rounding halfway cases away from zero.
    ///
    /// This variant indicates that the rounding should round to the nearest whole number, with
    /// halfway cases rounded away from zero.
    RoundHalfUp,
    /// Rounds up to the nearest whole number.
    ///
    /// This variant indicates that the rounding should always go towards positive infinity.
    RoundUp,
}

lazy_static! {
    /// The maximum value representable by a `BigInt` in this context, equivalent to the maximum value of a `U256`.
    pub static ref MAX_UINT256: BigInt =
        BigInt::from_bytes_be(Sign::Plus, &U256::MAX.to_be_bytes::<32>());
}
