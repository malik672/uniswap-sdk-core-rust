use crate::prelude::*;

/// Represents the native currency of the chain on which it resides
pub trait NativeCurrency: Currency {
    fn is_native(&self) -> bool {
        true
    }

    fn is_token(&self) -> bool {
        false
    }
}
