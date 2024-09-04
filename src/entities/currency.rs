use crate::prelude::*;

/// Trait for representing a currency in the Uniswap Core SDK.
pub trait Currency: BaseCurrency {
    /// Returns whether the currency is native to the chain and must be wrapped (e.g. Ether)
    fn is_native(&self) -> bool;

    /// Returns the address of the currency.
    fn address(&self) -> Address;

    /// Returns whether this currency is functionally equivalent to the other currency
    fn equals(&self, other: &impl Currency) -> bool;

    /// Returns a Token that represents the wrapped equivalent of the native currency
    fn wrapped(&self) -> &Token;
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
