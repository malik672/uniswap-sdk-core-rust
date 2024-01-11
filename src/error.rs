use crate::prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Chain IDs do not match: {0} and {1}")]
    ChainIdMismatch(u32, u32),

    #[error("Addresses are equal")]
    EqualAddresses,

    #[error("amount has exceeded MAX_UINT256")]
    MaxUint,

    #[error("not equal: {0}")]
    NotEqual(String),

    //Custo
    #[error("incorrect: {0}")]
    Incorrect(String),
}
