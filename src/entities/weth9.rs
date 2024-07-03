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
    pub fn new() -> Self {
        let mut tokens = FxHashMap::default();

        // Insert predefined WETH tokens for different chains.
        tokens.insert(
            1,
            token!(
                1,
                "C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            3,
            token!(
                3,
                "c778417E063141139Fce010982780140Aa0cD5Ab",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            4,
            token!(
                4,
                "c778417E063141139Fce010982780140Aa0cD5Ab",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            5,
            token!(
                5,
                "B4FBF271143F4FBf7B91A5ded31805e42b2208d6",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            42,
            token!(
                42,
                "d0A1E359811322d97991E03f863a0C30C2cF029C",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            10,
            token!(
                10,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            69,
            token!(
                69,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            42161,
            token!(
                42161,
                "82aF49447D8a07e3bd95BD0d56f35241523fBab1",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            421611,
            token!(
                421611,
                "B47e6A5f8b33b3F17603C83a0535A9dcD7E32681",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            8453,
            token!(
                8453,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            ),
        );
        tokens.insert(
            56,
            token!(
                56,
                "bb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c",
                18,
                "WBNB",
                "Wrapped BNB"
            ),
        );
        tokens.insert(
            137,
            token!(
                137,
                "0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
                18,
                "WMATIC",
                "Wrapped MATIC"
            ),
        );
        tokens.insert(
            43114,
            token!(
                43114,
                "B31f66AA3C1e785363F0875A1B74E27b85FD66c7",
                18,
                "WAVAX",
                "Wrapped AVAX"
            ),
        );

        Self { tokens }
    }

    /// Retrieves the WETH token for a specific chain ID, if it exists.
    ///
    /// # Arguments
    ///
    /// * `chain_id`: The chain ID for which to retrieve the WETH token.
    ///
    /// Returns: `Some(Token)` if the token exists, `None` otherwise.
    pub fn get(&self, chain_id: u64) -> Option<&Token> {
        self.tokens.get(&chain_id)
    }
}
