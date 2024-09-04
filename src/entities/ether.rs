use crate::prelude::*;

/// Ether is the main usage of a 'native' currency, i.e., for Ethereum mainnet and all testnets.
/// Represents the native currency of the blockchain.
pub type Ether = CurrencyLike<Option<Token>>;

impl Currency for Ether {
    /// Checks if the currency is native to the blockchain.
    #[inline]
    fn is_native(&self) -> bool {
        true
    }

    /// Retrieves the address associated with the currency.
    #[inline]
    fn address(&self) -> Address {
        self.wrapped().address()
    }

    /// Checks if the currency is equal to another currency.
    #[inline]
    fn equals(&self, other: &impl Currency) -> bool {
        other.is_native() && self.chain_id() == other.chain_id()
    }

    /// Returns the wrapped token representation of the currency.
    #[inline]
    fn wrapped(&self) -> &Token {
        match &self.meta {
            Some(weth) => weth,
            None => panic!("WRAPPED"),
        }
    }
}

impl Ether {
    /// Creates a new instance of [`Ether`] with the specified chain ID.
    #[inline]
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id,
            decimals: 18,
            symbol: Some("ETH".to_string()),
            name: Some("Ether".to_string()),
            meta: WETH9::on_chain(chain_id),
        }
    }

    /// Retrieves or creates an [`Ether`] instance for the specified chain ID.
    #[inline]
    pub fn on_chain(chain_id: u64) -> Self {
        Self::new(chain_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_constructor_uses_cache() {
        assert_eq!(Ether::on_chain(1), Ether::on_chain(1));
    }

    #[test]
    fn test_caches_once_per_chain_id() {
        assert_ne!(Ether::on_chain(1), Ether::on_chain(2));
    }

    #[test]
    fn test_equals_returns_false_for_different_chains() {
        assert!(!Ether::on_chain(1).equals(&Ether::on_chain(2)));
    }

    #[test]
    fn test_equals_returns_true_for_same_chains() {
        assert!(Ether::on_chain(1).equals(&Ether::on_chain(1)));
    }
}
