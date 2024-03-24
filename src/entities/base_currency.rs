use std::ops::Deref;

use alloy_primitives::ChainId;

#[derive(Clone, PartialEq, Debug)]

/// `CurrencyLike` is a generic struct representing a currency with a specific chain ID,
/// decimals, symbol, name, and additional metadata.
///
/// This struct is used to abstract the details of different currencies, allowing for
/// a unified way to handle various types of currencies in the Uniswap SDK Core.
///
/// # Generics
///
/// - `M`: The type of the additional metadata associated with the currency.
pub struct CurrencyLike<M> {
    /// The chain ID on which this currency resides.
    ///
    /// This identifies the blockchain network where the currency is used.
    pub chain_id: ChainId,

    /// The number of decimal places the currency can be divided into.
    ///
    /// This is used to represent the smallest unit of the currency.
    pub decimals: u8,

    /// The symbol of the currency, i.e., a short textual non-unique identifier.
    ///
    /// This is a common abbreviation used to represent the currency.
    pub symbol: Option<String>,

    /// The name of the currency, i.e., a descriptive textual non-unique identifier.
    ///
    /// This is a more detailed name used to represent the currency.
    pub name: Option<String>,

    /// Additional metadata associated with the currency.
    ///
    /// This can include various details specific to the currency, such as contract addresses or
    /// other relevant information.
    pub meta: M,
}

/// A currency is any fungible financial instrument, including Ether, all ERC20 tokens, and other
/// chain-native currencies
pub trait BaseCurrency: Clone {
    /// The chain ID on which this currency resides
    fn chain_id(&self) -> ChainId;

    /// The decimals used in representing currency amounts
    fn decimals(&self) -> u8;

    /// The symbol of the currency, i.e. a short textual non-unique identifier
    fn symbol(&self) -> Option<String>;

    /// The name of the currency, i.e. a descriptive textual non-unique identifier
    fn name(&self) -> Option<String>;
}

impl<M: Clone> BaseCurrency for CurrencyLike<M> {
    fn chain_id(&self) -> ChainId {
        self.chain_id
    }

    fn decimals(&self) -> u8 {
        self.decimals
    }

    fn symbol(&self) -> Option<String> {
        self.symbol.clone()
    }

    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}

/// Implement [`Deref`] to allow direct access to the metadata of the currency
impl<M> Deref for CurrencyLike<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}
