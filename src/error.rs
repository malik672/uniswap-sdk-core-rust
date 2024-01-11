use crate::prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{field} must be greater than 0")]
    ChainIdError { field: &'static str },

    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Chain IDs do not match: {0} and {1}")]
    ChainIdMismatch(u32, u32),

    #[error("Addresses are equal")]
    EqualAddresses,

    #[error("amount has exceeded MAX_UINT256")]
    MaxUint,

    #[error("Error creating: {0}")]
    CreationError(String),

    #[error("Can't get the fractional amount: {0}")]
    CreateFractionalError(String),

    #[error("not equal: {0}")]
    NotEqual(String),

    #[error("Denominator is 0")]
    DenominatorIsZero,

    #[error("incorrect: {0}")]
    Incorrect(String),
}
