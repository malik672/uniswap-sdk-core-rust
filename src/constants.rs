use lazy_static::lazy_static;
use num_bigint::BigInt;
use num_traits::Num;

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
    pub static ref MAX_UINT256: BigInt = BigInt::from_str_radix(
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        16
    )
    .unwrap();
}
