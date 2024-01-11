#[derive(Clone, PartialEq, Debug)]
pub struct CurrencyLike<M: Clone> {
    pub chain_id: u32,
    pub decimals: u8,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub meta: M,
}

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
}

// Implementation of methods for CurrencyLike
impl<M: Clone> BaseCurrency for CurrencyLike<M> {
    fn chain_id(&self) -> u32 {
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
