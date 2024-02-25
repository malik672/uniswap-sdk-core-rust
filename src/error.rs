use crate::prelude::*;

/// Represents errors that can occur in the context of currency operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Indicates a mismatch in chain IDs.
    #[error("Chain IDs do not match: {0} and {1}")]
    ChainIdMismatch(u64, u64),

    /// Indicates that two addresses are equal.
    #[error("Addresses are equal")]
    EqualAddresses,

    /// Indicates that an amount has exceeded MAX_UINT256.
    #[error("amount has exceeded MAX_UINT256")]
    MaxUint,

    /// Indicates that two entities are not equal.
    #[error("not equal")]
    NotEqual(),

    /// Custom error for incorrect input.
    #[error("incorrect")]
    Incorrect(),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that `Error::ChainIdMismatch` displays the correct error message.
    #[test]
    fn test_chain_id_mismatch_error() {
        let error = Error::ChainIdMismatch(1, 2);
        assert_eq!(error.to_string(), "Chain IDs do not match: 1 and 2");
    }

    /// Test that `Error::EqualAddresses` displays the correct error message.
    #[test]
    fn test_equal_addresses_error() {
        let error = Error::EqualAddresses;
        assert_eq!(error.to_string(), "Addresses are equal");
    }

    /// Test that `Error::MaxUint` displays the correct error message.
    #[test]
    fn test_max_uint_error() {
        let error = Error::MaxUint;
        assert_eq!(error.to_string(), "amount has exceeded MAX_UINT256");
    }

    /// Test that `Error::NotEqual` displays the correct error message.
    #[test]
    fn test_not_equal_error() {
        let error = Error::NotEqual();
        assert_eq!(error.to_string(), "not equal");
    }

    /// Test that `Error::Incorrect` displays the correct error message.
    #[test]
    fn test_incorrect_error() {
        let error = Error::Incorrect();
        assert_eq!(error.to_string(), "incorrect");
    }
}
