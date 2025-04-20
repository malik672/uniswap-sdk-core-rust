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
        const CHAIN_IDS: [u64; 25] = [
            1, 11155111, 3, 4, 5, 42, 10, 69, 11155420, 42161, 421611, 421614, 8453, 84532, 56,
            137, 43114, 7777777, 81457, 324, 480, 1301, 130, 10143, 1868,
        ];
        let tokens = HashMap::from_iter(
            CHAIN_IDS
                .into_iter()
                .map(|chain_id| (chain_id, Self::on_chain(chain_id).unwrap())),
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
            11155111 => Some(token!(
                11155111,
                "0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14",
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
            84532 => Some(token!(
                84532,
                "0x4200000000000000000000000000000000000006",
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
            130 => Some(token!(
                130,
                "0x4200000000000000000000000000000000000006",
                18,
                "WETH",
                "Wrapped Ether"
            )),
            10143 => Some(token!(
                10143,
                "0x760AfE86e5de5fa0Ee542fc7B7B713e1c5425701",
                18,
                "WMON",
                "Wrapped Monad"
            )),
            1868 => Some(token!(
                1868,
                "0x4200000000000000000000000000000000000006",
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
