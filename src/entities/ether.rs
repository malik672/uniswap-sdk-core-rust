use super::{base_currency::BaseCurrency, token::Token};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref ETHER_CACHE: Mutex<HashMap<u32, Ether>> = Mutex::new(HashMap::new());
}

/// Ether is the main usage of a 'native' currency, i.e. for Ethereum mainnet and all testnets
#[derive(Clone, PartialEq)]
pub struct Ether {
    base_currency: BaseCurrency,
    wrapped: Token,
}

impl Ether {
    pub fn new(chain_id: u32) -> Self {
        Self {
            base_currency: BaseCurrency::new(
                chain_id,
                18,
                Some("Ether".to_string()),
                Some("ETH".to_string()),
            ),
            wrapped: Token::new(
                chain_id,
                "0x".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        }
    }

    pub fn wrapped(&self) -> &Token {
        &self.wrapped
    }

    pub fn on_chain(chain_id: u32) -> Self {
        let mut cache = ETHER_CACHE.lock().unwrap();
        match cache.get(&chain_id) {
            Some(ether) => ether.clone(),
            None => {
                let ether = Ether::new(chain_id);
                cache.insert(chain_id, ether.clone());
                ether
            }
        }
    }

    pub fn equals(&self, other: &BaseCurrency) -> bool {
        other.is_native && other.chain_id == self.base_currency.chain_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_constructor_uses_cache() {
        assert!(Ether::on_chain(1) == Ether::on_chain(1));
    }

    #[test]
    fn test_caches_once_per_chain_id() {
        assert!(Ether::on_chain(1) != Ether::on_chain(2));
    }

    #[test]
    fn test_equals_returns_false_for_different_chains() {
        assert!(!Ether::on_chain(1).equals(&Ether::on_chain(2).base_currency));
    }

    #[test]
    fn test_equals_returns_true_for_same_chains() {
        assert!(Ether::on_chain(1).equals(&Ether::on_chain(1).base_currency));
    }
}
