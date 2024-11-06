/// Custom error types that are used throughout the SDK to handle various error conditions.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Triggers when the compared chain IDs do not match.
    #[error("chain IDs do not match: {0} and {1}")]
    ChainIdMismatch(u64, u64),

    /// Triggers when compared addresses are the same.
    #[error("addresses are equal")]
    EqualAddresses,

    /// Triggers when it tries to exceed [`alloy_primitives::U256::MAX`].
    #[error("amount exceeds U256::MAX")]
    UintOverflow,

    /// Triggers when the currency values are not equal.
    #[error("currency values are not equal")]
    CurrencyMismatch,

    /// Triggers when the value is invalid.
    #[error("{0}")]
    Invalid(&'static str),
}

#[cfg(all(feature = "std", test))]
mod tests {
    use super::*;

    #[test]
    fn test_chain_id_mismatch_error() {
        let error = Error::ChainIdMismatch(1, 2);
        assert_eq!(error.to_string(), "chain IDs do not match: 1 and 2");
    }

    #[test]
    fn test_equal_addresses_error() {
        let error = Error::EqualAddresses;
        assert_eq!(error.to_string(), "addresses are equal");
    }

    #[test]
    fn test_max_uint_error() {
        let error = Error::UintOverflow;
        assert_eq!(error.to_string(), "amount exceeds U256::MAX");
    }

    #[test]
    fn test_not_equal_error() {
        let error = Error::CurrencyMismatch;
        assert_eq!(error.to_string(), "currency values are not equal");
    }

    #[test]
    fn test_incorrect_error() {
        let error = Error::Invalid("invalid");
        assert_eq!(error.to_string(), "invalid");
    }
}
