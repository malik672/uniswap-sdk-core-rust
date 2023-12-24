use super::{native_currency::NativeCurrency, token::Token};

#[derive(Clone, PartialEq)]
pub enum Currency {
    NativeCurrency(NativeCurrency),
    Token(Token),
}
