use crate::{prelude::*, token};
use alloc::string::ToString;

/// Represents the WETH9 contract and provides information about WETH tokens on different Ethereum
/// chains.
#[derive(Clone, PartialEq, Debug)]
pub struct WETH9 {
    /// A mapping of chain IDs to corresponding WETH tokens.
    tokens: HashMap<u64, Token>,
}

/// Default implementation for [`WETH9`], creating an instance with predefined WETH tokens on
/// various chains.
impl Default for WETH9 {
    #[inline]
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
    #[must_use]
    pub fn new() -> Self {
        let tokens = HashMap::from_iter([
            (1, Self::on_chain(1).unwrap()),
            (3, Self::on_chain(3).unwrap()),
            (4, Self::on_chain(4).unwrap()),
            (5, Self::on_chain(5).unwrap()),
            (42, Self::on_chain(42).unwrap()),
            (10, Self::on_chain(10).unwrap()),
            (69, Self::on_chain(69).unwrap()),
            (11155420, Self::on_chain(11155420).unwrap()),
            (42161, Self::on_chain(42161).unwrap()),
            (421611, Self::on_chain(421611).unwrap()),
            (421614, Self::on_chain(421614).unwrap()),
            (8453, Self::on_chain(8453).unwrap()),
            (56, Self::on_chain(56).unwrap()),
            (137, Self::on_chain(137).unwrap()),
            (43114, Self::on_chain(43114).unwrap()),
            (7777777, Self::on_chain(7777777).unwrap()),
            (81457, Self::on_chain(81457).unwrap()),
            (324, Self::on_chain(324).unwrap()),
            (480, Self::on_chain(480).unwrap()),
            (1301, Self::on_chain(1301).unwrap()),
        ]);
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
    #[must_use]
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
            11155420 => Some(token!(
                11155420,
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
            421614 => Some(token!(
                421614,
                "980B62Da83eFf3D4576C647993b0c1D7faf17c73",
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
            7777777 => Some(token!(
                7777777,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            81457 => Some(token!(
                81457,
                "4300000000000000000000000000000000000004",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            324 => Some(token!(
                324,
                "5AEa5775959fBC2557Cc8789bC1bf90A239D9a91",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            480 => Some(token!(
                480,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            1301 => Some(token!(
                1301,
                "4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
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
    #[must_use]
    pub fn get(&self, chain_id: u64) -> Option<&Token> {
        self.tokens.get(&chain_id)
    }
}
