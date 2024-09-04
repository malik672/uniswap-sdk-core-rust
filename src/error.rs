/// Represents errors that can occur in the context of currency operations.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    /// Triggers when the compared chain IDs do not match.
    #[cfg_attr(feature = "std", error("chain IDs do not match: {0} and {1}"))]
    ChainIdMismatch(u64, u64),

    /// Triggers when compared addresses are the same.
    #[cfg_attr(feature = "std", error("addresses are equal"))]
    EqualAddresses,

    /// Triggers when it tries to exceed the max uint.
    #[cfg_attr(feature = "std", error("amount has exceeded MAX_UINT256"))]
    MaxUint,

    /// Triggers when the compared values are not equal.
    #[cfg_attr(feature = "std", error("not equal"))]
    NotEqual,

    /// Triggers when the value is invalid.
    #[cfg_attr(feature = "std", error("invalid"))]
    Invalid,
}

#[cfg(all(feature = "std", test))]
mod tests {
    use super::*;

    /// Test that `Error::ChainIdMismatch` displays the correct error message.
    #[test]
    fn test_chain_id_mismatch_error() {
        let error = Error::ChainIdMismatch(1, 2);
        assert_eq!(error.to_string(), "chain IDs do not match: 1 and 2");
    }

    /// Test that `Error::EqualAddresses` displays the correct error message.
    #[test]
    fn test_equal_addresses_error() {
        let error = Error::EqualAddresses;
        assert_eq!(error.to_string(), "addresses are equal");
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
        let error = Error::NotEqual;
        assert_eq!(error.to_string(), "not equal");
    }

    /// Test that `Error::Invalid` displays the correct error message.
    #[test]
    fn test_incorrect_error() {
        let error = Error::Invalid;
        assert_eq!(error.to_string(), "invalid");
    }
}
