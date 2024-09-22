use crate::prelude::*;

/// Represents the native currency of the chain on which it resides
pub trait NativeCurrency: BaseCurrency {
    #[inline]
    fn is_native(&self) -> bool {
        true
    }

    #[inline]
    fn is_token(&self) -> bool {
        false
    }
}

impl<M> NativeCurrency for CurrencyLike<true, M> {}
