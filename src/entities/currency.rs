use super::{token::Token, native_currency::NativeCurrency};

#[derive(Clone, PartialEq)]
pub enum Currency {
   NativeCurrency(NativeCurrency),
    Token(Token),
}