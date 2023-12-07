use num_bigint::BigInt;
use num_traits::Num;
use lazy_static::lazy_static;

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
    pub static ref MaxUint256: BigInt = BigInt::from_str_radix("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 16).unwrap();
}