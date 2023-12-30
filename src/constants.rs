use crate::prelude::*;

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
