use super::{
    base_currency::{BaseCurrency, CurrencyLike},
    currency::CurrencyTrait,
    token::Token,
    weth9::WETH9,
};
use alloy_primitives::Address;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref ETHER_CACHE: Mutex<HashMap<u32, Ether>> = Mutex::new(HashMap::new());
}

/// Ether is the main usage of a 'native' currency, i.e. for Ethereum mainnet and all testnets
pub type Ether = CurrencyLike<()>;

impl CurrencyTrait for Ether {
    fn is_native(&self) -> bool {
        true
    }

    fn address(&self) -> Address {
        self.wrapped().address()
    }

    fn equals(&self, other: &impl CurrencyTrait) -> bool {
        match other.is_native() {
            true => self.chain_id() == other.chain_id(),
            _ => false,
        }
    }

    fn wrapped(&self) -> Token {
        match WETH9::default().get(self.chain_id()) {
            Some(weth9) => weth9.clone(),
            None => panic!("WRAPPED"),
        }
    }
}

impl Ether {
    pub fn new(chain_id: u32) -> Self {
        Self {
            chain_id,
            decimals: 18,
            symbol: Some("ETH".to_string()),
            name: Some("Ether".to_string()),
            meta: (),
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
