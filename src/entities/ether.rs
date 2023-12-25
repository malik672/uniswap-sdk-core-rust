use super::{base_currency::BaseCurrency, currency::CurrencyTrait, token::Token};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref ETHER_CACHE: Mutex<HashMap<u32, Ether>> = Mutex::new(HashMap::new());
}

/// Ether is the main usage of a 'native' currency, i.e. for Ethereum mainnet and all testnets
#[derive(Clone, PartialEq)]
pub struct Ether {
    pub chain_id: u32,
    pub decimals: u32,
    pub symbol: Option<String>,
    pub name: Option<String>,
}

impl Ether {
    pub fn new(chain_id: u32) -> Self {
        Self {
            chain_id,
            decimals: 18,
            symbol: Some("ETH".to_string()),
            name: Some("Ether".to_string()),
        }
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
}

impl BaseCurrency for Ether {
    fn chain_id(&self) -> u32 {
        self.chain_id
    }

    fn decimals(&self) -> u32 {
        self.decimals
    }

    fn symbol(&self) -> Option<String> {
        self.symbol.clone()
    }

    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    fn equals(&self, other: &impl CurrencyTrait) -> bool {
        match other.is_native() {
            true => self.chain_id() == other.chain_id(),
            _ => false,
        }
    }

    fn wrapped(&self) -> Token {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::currency::Currency;

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
        assert!(!Ether::on_chain(1).equals(&Currency::NativeCurrency(Ether::on_chain(2))));
    }

    #[test]
    fn test_equals_returns_true_for_same_chains() {
        assert!(Ether::on_chain(1).equals(&Currency::NativeCurrency(Ether::on_chain(1))));
    }
}
