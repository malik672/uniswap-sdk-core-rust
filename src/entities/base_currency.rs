use std::ops::Deref;

use alloy_primitives::ChainId;

/// `CurrencyLike` is a generic struct representing a currency with a specific chain ID,
/// decimals, symbol, name, and additional metadata.
#[derive(Clone, PartialEq, Debug)]
pub struct CurrencyLike<M> {
    /// The chain ID on which this currency resides
    pub chain_id: ChainId,

    /// represents the deciamls for the prticular currency
    pub decimals: u8,

    /// The symbol of the currency, i.e. a short textual non-unique identifier
    pub symbol: Option<String>,

    /// The name of the currency, i.e. a descriptive textual non-unique identifier
    pub name: Option<String>,

    /// Metadata associated with the currency
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
