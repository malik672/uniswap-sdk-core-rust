use crate::prelude::{BigDecimal, BigInt, BigUint};
use alloy_primitives::U256;
use fastnum::decimal::{Context, Sign};

#[inline]
#[must_use]
pub const fn to_big_decimal(value: BigInt) -> BigDecimal {
    match value.is_negative() {
        false => BigDecimal::from_parts(value.to_bits(), 0, Sign::Plus, Context::default()),
        true => BigDecimal::from_parts(value.to_bits(), 0, Sign::Minus, Context::default()),
    }
}

#[inline]
#[must_use]
pub const fn to_big_uint(x: U256) -> BigUint {
    BigUint::from_le_slice(x.as_le_slice()).unwrap()
}

#[inline]
#[must_use]
pub const fn to_big_int(x: U256) -> BigInt {
    BigInt::from_bits(to_big_uint(x))
}
