use crate::prelude::*;
use alloy_primitives::ChainId;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Currency {
    NativeCurrency(Ether),
    Token(Token),
}

/// [`CurrencyLike`] is a generic struct representing a currency with a specific chain ID,
/// decimals, symbol, name, and additional metadata.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CurrencyLike<const IS_NATIVE: bool, M> {
    /// The chain ID on which this currency resides
    pub chain_id: ChainId,

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

macro_rules! match_currency_method {
    ($method:ident, $return_type:ty) => {
        #[inline]
        fn $method(&self) -> $return_type {
            match self {
                Currency::NativeCurrency(ether) => ether.$method(),
                Currency::Token(token) => token.$method(),
            }
        }
    };
}

macro_rules! impl_base_currency_core {
    ($($currency:ty),*) => {
        $(
            impl BaseCurrencyCore for $currency {
                #[inline]
                fn is_native(&self) -> bool {
                    matches!(self, Currency::NativeCurrency(_))
                }

                #[inline]
                fn is_token(&self) -> bool {
                    matches!(self, Currency::Token(_))
                }

                match_currency_method!(chain_id, ChainId);

                match_currency_method!(decimals, u8);

                match_currency_method!(symbol, Option<&String>);

                match_currency_method!(name, Option<&String>);
            }
        )*
    };
}

macro_rules! impl_base_currency {
    ($($currency:ty),*) => {
        $(
            impl BaseCurrency for $currency {
                #[inline]
                fn equals(&self, other: &impl BaseCurrency) -> bool {
                    match self {
                        Currency::NativeCurrency(ether) => ether.equals(other),
                        Currency::Token(token) => token.equals(other),
                    }
                }

                match_currency_method!(wrapped, &Token);
            }
        )*
    };
}

impl_base_currency_core!(Currency, &Currency);
impl_base_currency!(Currency, &Currency);

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
