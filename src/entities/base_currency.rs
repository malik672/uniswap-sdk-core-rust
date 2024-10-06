use crate::prelude::*;
use alloy_primitives::ChainId;

/// A currency is any fungible financial instrument, including Ether, all ERC20 tokens, and other
/// chain-native currencies
pub trait BaseCurrency: BaseCurrencyCore + Clone {
    /// Returns the address of the currency.
    #[inline]
    fn address(&self) -> Address {
        self.wrapped().address
    }

    /// Returns whether this currency is functionally equivalent to the other currency
    fn equals(&self, other: &impl BaseCurrency) -> bool;

    /// Returns a Token that represents the wrapped equivalent of the native currency
    fn wrapped(&self) -> &Token;
}

pub trait BaseCurrencyCore {
    /// Returns whether the currency is native to the chain and must be wrapped (e.g. Ether)
    fn is_native(&self) -> bool;

    /// Returns whether the currency is a token that is usable in Uniswap without wrapping
    fn is_token(&self) -> bool;

    /// The chain ID on which this currency resides
    fn chain_id(&self) -> ChainId;

    /// The decimals used in representing currency amounts
    fn decimals(&self) -> u8;

    /// The symbol of the currency, i.e. a short textual non-unique identifier
    fn symbol(&self) -> Option<&String>;

    /// The name of the currency, i.e. a descriptive textual non-unique identifier
    fn name(&self) -> Option<&String>;
}

macro_rules! impl_base_currency_core {
    ($($currency:ty),*) => {
        $(
            impl<const IS_NATIVE: bool, M> BaseCurrencyCore for $currency {
                #[inline]
                fn is_native(&self) -> bool {
                    IS_NATIVE
                }

                #[inline]
                fn is_token(&self) -> bool {
                    !IS_NATIVE
                }

                #[inline]
                fn chain_id(&self) -> ChainId {
                    self.chain_id
                }

                #[inline]
                fn decimals(&self) -> u8 {
                    self.decimals
                }

                #[inline]
                fn symbol(&self) -> Option<&String> {
                    self.symbol.as_ref()
                }

                #[inline]
                fn name(&self) -> Option<&String> {
                    self.name.as_ref()
                }
            }
        )*
    };
}

impl_base_currency_core!(CurrencyLike<IS_NATIVE, M>, &CurrencyLike<IS_NATIVE, M>);
