use crate::{prelude::*, token};

/// Represents the WETH9 contract and provides information about WETH tokens on different Ethereum
/// chains.
#[derive(Clone, PartialEq, Debug)]
pub struct WETH9 {
    /// A mapping of chain IDs to corresponding WETH tokens.
    tokens: FxHashMap<u64, Token>,
}

/// Default implementation for [`WETH9`], creating an instance with predefined WETH tokens on
/// various chains.
impl Default for WETH9 {
    fn default() -> Self {
        Self::new()
    }
}

impl WETH9 {
    /// Creates a new instance of `WETH9` with predefined WETH tokens for various chains.
    ///
    /// This function initializes a `WETH9` struct with a predefined set of WETH tokens
    /// for different Ethereum chains. It's useful for quickly setting up a `WETH9`
    /// instance without manually inserting each token.
    ///
    /// # Returns
    ///
    /// A new `WETH9` instance with predefined WETH tokens.
    #[inline]
    pub fn new() -> Self {
        let mut tokens = FxHashMap::default();
        tokens.insert(1, Self::on_chain(1).unwrap());
        tokens.insert(3, Self::on_chain(3).unwrap());
        tokens.insert(4, Self::on_chain(4).unwrap());
        tokens.insert(5, Self::on_chain(5).unwrap());
        tokens.insert(42, Self::on_chain(42).unwrap());
        tokens.insert(10, Self::on_chain(10).unwrap());
        tokens.insert(69, Self::on_chain(69).unwrap());
        tokens.insert(42161, Self::on_chain(42161).unwrap());
        tokens.insert(421611, Self::on_chain(421611).unwrap());
        tokens.insert(8453, Self::on_chain(8453).unwrap());
        tokens.insert(56, Self::on_chain(56).unwrap());
        tokens.insert(137, Self::on_chain(137).unwrap());
        tokens.insert(43114, Self::on_chain(43114).unwrap());
        Self { tokens }
    }

    /// Retrieves the WETH token for a specific chain ID, if it exists.
    ///
    /// # Arguments
    ///
    /// * `chain_id`: The chain ID for which to retrieve the WETH token.
    ///
    /// Returns: `Some(Token)` if the token exists, `None` otherwise.
    #[inline]
    pub fn on_chain(chain_id: u64) -> Option<Token> {
        match chain_id {
            1 => Some(token!(
                1,
                "C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            3 => Some(token!(
                3,
                "c778417E063141139Fce010982780140Aa0cD5Ab",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            4 => Some(token!(
                4,
                "c778417E063141139Fce010982780140Aa0cD5Ab",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            5 => Some(token!(
                5,
                "B4FBF271143F4FBf7B91A5ded31805e42b2208d6",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            42 => Some(token!(
                42,
                "d0A1E359811322d97991E03f863a0C30C2cF029C",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            10 => Some(token!(
                10,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            69 => Some(token!(
                69,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            42161 => Some(token!(
                42161,
                "82aF49447D8a07e3bd95BD0d56f35241523fBab1",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            421611 => Some(token!(
                421611,
                "B47e6A5f8b33b3F17603C83a0535A9dcD7E32681",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            8453 => Some(token!(
                8453,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            56 => Some(token!(
                56,
                "bb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c",
                18,
                "WBNB",
                "Wrapped BNB"
            )),
            137 => Some(token!(
                137,
                "0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
                18,
                "WMATIC",
                "Wrapped MATIC"
            )),
            43114 => Some(token!(
                43114,
                "B31f66AA3C1e785363F0875A1B74E27b85FD66c7",
                18,
                "WAVAX",
                "Wrapped AVAX"
            )),
            _ => None,
        }
    }

    /// Retrieves the WETH token for a specific chain ID, if it exists.
    ///
    /// # Arguments
    ///
    /// * `chain_id`: The chain ID for which to retrieve the WETH token.
    ///
    /// Returns: `Some(Token)` if the token exists, `None` otherwise.
    #[inline]
    pub fn get(&self, chain_id: u64) -> Option<&Token> {
        self.tokens.get(&chain_id)
    }
}
