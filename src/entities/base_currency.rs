use super::{currency::CurrencyTrait, token::Token};

/// A currency is any fungible financial instrument, including Ether, all ERC20 tokens, and other chain-native currencies
pub trait BaseCurrency: Clone {
    /// The chain ID on which this currency resides
    fn chain_id(&self) -> u32;

    /// The decimals used in representing currency amounts
    fn decimals(&self) -> u8;

    /// The symbol of the currency, i.e. a short textual non-unique identifier
    fn symbol(&self) -> Option<String>;

    /// The name of the currency, i.e. a descriptive textual non-unique identifier
    fn name(&self) -> Option<String>;

    /// Returns whether this currency is functionally equivalent to the other currency
    ///
    /// # Arguments
    ///
    /// * `other`: the other currency
    ///
    fn equals(&self, other: &impl CurrencyTrait) -> bool;

    /// Return the wrapped version of this currency that can be used with the Uniswap contracts.
    /// Currencies must implement this to be used in Uniswap
    fn wrapped(&self) -> Token;
}
