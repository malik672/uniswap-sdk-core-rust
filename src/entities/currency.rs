use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
/// Represents a currency in the Uniswap SDK.
///
/// This enum can represent either a native currency (like Ether) or a token.
pub enum Currency {
    /// Represents a native currency.
    NativeCurrency(Ether),
    /// Represents a token.
    Token(Token),
}

/// Trait for representing a currency in the Uniswap SDK.
///
/// This trait provides methods for interacting with currencies, whether they are native currencies
/// like Ether or tokens. Implementations of this trait must provide functionality for determining
/// if a currency is native, getting its address, checking equality with another currency, and
/// wrapping the currency for use with Uniswap contracts.
pub trait CurrencyTrait: BaseCurrency {
    /// Returns whether the currency is native to the chain and must be wrapped (e.g. Ether)
    fn is_native(&self) -> bool;

    /// Returns the address of the currency.
    ///
    /// This method returns the address associated with the currency, whether it's a native currency
    /// or a token.
    ///
    /// # Returns
    ///
    /// * `Address` - The address of the currency.
    fn address(&self) -> Address;

    /// Returns whether this currency is functionally equivalent to the other currency
    ///
    /// # Arguments
    ///
    /// * `other`: the other currency
    fn equals(&self, other: &impl CurrencyTrait) -> bool;

    /// Return the wrapped version of this currency that can be used with the Uniswap contracts.
    /// Currencies must implement this to be used in Uniswap
    fn wrapped(&self) -> Token;
}

impl CurrencyTrait for Currency {
    /// Returns a bool indicating whether the currency is native or not
    fn is_native(&self) -> bool {
        match self {
            Currency::NativeCurrency(_) => true,
            Currency::Token(_) => false,
        }
    }

    /// Accessor method for retrieving either the NativeCurrency or Token address
    fn address(&self) -> Address {
        match self {
            Currency::NativeCurrency(native_currency) => native_currency.address(),
            Currency::Token(token) => token.address(),
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

impl BaseCurrency for Currency {
    fn chain_id(&self) -> u64 {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token;

    const ADDRESS_ZERO: &str = "0x0000000000000000000000000000000000000000";
    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";

    lazy_static! {
        static ref TOKEN0: Token = token!(1, ADDRESS_ZERO, 18);
        static ref TOKEN1: Token = token!(1, ADDRESS_ONE, 18);
    }

    #[test]
    fn equals_ether_on_same_chains_is_ether() {
        assert!(Ether::on_chain(1).equals(&Ether::on_chain(1)));
    }

    #[test]
    fn equals_ether_is_not_token0() {
        assert!(!Ether::on_chain(1).equals(&TOKEN0.clone()));
    }

    #[test]
    fn equals_token1_is_not_token0() {
        assert!(!TOKEN1.equals(&TOKEN0.clone()));
    }

    #[test]
    fn equals_token0_is_token0() {
        assert!(TOKEN0.equals(&TOKEN0.clone()));
    }

    #[test]
    fn equals_token0_is_equal_to_another_token0() {
        assert!(TOKEN0.equals(&token!(1, ADDRESS_ZERO, 18, "symbol", "name")));
    }
}
