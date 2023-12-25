use super::{base_currency::BaseCurrency, ether::Ether, token::Token};
use alloy_primitives::Address;

#[derive(Clone, PartialEq)]
pub enum Currency {
    NativeCurrency(Ether),
    Token(Token),
}

pub trait CurrencyTrait: BaseCurrency {
    /// Returns whether the currency is native to the chain and must be wrapped (e.g. Ether)
    fn is_native(&self) -> bool;

    fn address(&self) -> Address;
}

impl CurrencyTrait for Currency {
    fn is_native(&self) -> bool {
        match self {
            Currency::NativeCurrency(_) => true,
            Currency::Token(_) => false,
        }
    }

    fn address(&self) -> Address {
        match self {
            Currency::NativeCurrency(native_currency) => native_currency.address(),
            Currency::Token(token) => token.address(),
        }
    }
}

impl BaseCurrency for Currency {
    fn chain_id(&self) -> u32 {
        match self {
            Currency::NativeCurrency(native_currency) => native_currency.chain_id(),
            Currency::Token(token) => token.chain_id(),
        }
    }

    fn decimals(&self) -> u8 {
        match self {
            Currency::NativeCurrency(native_currency) => native_currency.decimals(),
            Currency::Token(token) => token.decimals(),
        }
    }

    fn symbol(&self) -> Option<String> {
        match self {
            Currency::NativeCurrency(native_currency) => native_currency.symbol(),
            Currency::Token(token) => token.symbol(),
        }
    }

    fn name(&self) -> Option<String> {
        match self {
            Currency::NativeCurrency(native_currency) => native_currency.name(),
            Currency::Token(token) => token.name(),
        }
    }

    fn equals(&self, other: &impl CurrencyTrait) -> bool {
        match self {
            Currency::NativeCurrency(native_currency) => native_currency.equals(other),
            Currency::Token(token) => token.equals(other),
        }
    }

    fn wrapped(&self) -> Token {
        match self {
            Currency::NativeCurrency(native_currency) => native_currency.wrapped(),
            Currency::Token(token) => token.clone(),
        }
    }
}
