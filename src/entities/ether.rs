use crate::prelude::*;
use alloc::string::ToString;

/// Ether is the main usage of a 'native' currency, i.e., for Ethereum mainnet and all testnets.
/// Represents the native currency of the blockchain.
pub type Ether = CurrencyLike<true, Option<Token>>;

macro_rules! impl_base_currency {
    ($($ether:ty),*) => {
        $(
            impl BaseCurrency for $ether {
                #[inline]
                fn equals(&self, other: &impl BaseCurrency) -> bool {
                    other.is_native() && self.chain_id() == other.chain_id()
                }

                #[inline]
                fn wrapped(&self) -> &Token {
                    match &self.meta {
                        Some(weth) => weth,
                        None => panic!("WRAPPED"),
                    }
                }
            }
        )*
    };
}

impl_base_currency!(Ether, &Ether);

impl Ether {
    /// Creates a new instance of [`Ether`] with the specified chain ID.
    #[inline]
    #[must_use]
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
    #[must_use]
    pub fn on_chain(chain_id: u64) -> Self {
        Self::new(chain_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

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

    #[test]
    fn test_wrapped_returns_robinhood_weth() {
        assert_eq!(
            Ether::on_chain(4663).wrapped().address,
            address!("0x0Bd7D308f8E1639FAb988df18A8011f41EAcAD73")
        );
    }

    #[test]
    #[should_panic(expected = "WRAPPED")]
    fn test_wrapped_panics_for_arc() {
        Ether::on_chain(5042).wrapped();
    }
}
