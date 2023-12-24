use super::{native_currency::NativeCurrency, token::Token};

#[derive(Clone)]
pub enum Currency<'a> {
    NativeCurrency(&'a dyn NativeCurrency),
    Token(Token),
}
