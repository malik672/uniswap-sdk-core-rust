use crate::prelude::*;

/// Trait for representing a currency in the Uniswap Core SDK.
pub trait Currency: BaseCurrency + Clone {
    /// Returns the address of the currency.
    fn address(&self) -> Address;

    /// Returns whether this currency is functionally equivalent to the other currency
    fn equals(&self, other: &impl Currency) -> bool;

    /// Returns a Token that represents the wrapped equivalent of the native currency
    fn wrapped(&self) -> &Token;
}

/// [`CurrencyLike`] is a generic struct representing a currency with a specific chain ID,
/// decimals, symbol, name, and additional metadata.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CurrencyLike<const IS_NATIVE: bool, M> {
    /// The chain ID on which this currency resides
    pub chain_id: alloy_primitives::ChainId,

    /// The decimals for the particular currency
    pub decimals: u8,

    /// The symbol of the currency, i.e. a short textual non-unique identifier
    pub symbol: Option<String>,

    /// The name of the currency, i.e. a descriptive textual non-unique identifier
    pub name: Option<String>,

    /// Metadata associated with the currency
    pub meta: M,
}

/// Implement [`Deref`] to allow direct access to the metadata of the currency
impl<const IS_NATIVE: bool, M> Deref for CurrencyLike<IS_NATIVE, M> {
    type Target = M;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.meta
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
