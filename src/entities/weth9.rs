use super::token::Token;
use std::collections::HashMap;

pub struct WETH9 {
    tokens: HashMap<u32, Token>,
}

impl Default for WETH9 {
    fn default() -> Self {
        Self::new()
    }
}

impl WETH9 {
    pub fn new() -> Self {
        let mut tokens = HashMap::new();
        tokens.insert(
            1,
            Token::new(
                1,
                "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            3,
            Token::new(
                3,
                "0xc778417E063141139Fce010982780140Aa0cD5Ab".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            4,
            Token::new(
                4,
                "0xc778417E063141139Fce010982780140Aa0cD5Ab".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            5,
            Token::new(
                5,
                "0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            42,
            Token::new(
                42,
                "0xd0A1E359811322d97991E03f863a0C30C2cF029C".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            10,
            Token::new(
                10,
                "0x4200000000000000000000000000000000000006".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            69,
            Token::new(
                69,
                "0x4200000000000000000000000000000000000006".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            42161,
            Token::new(
                42161,
                "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            421611,
            Token::new(
                421611,
                "0xB47e6A5f8b33b3F17603C83a0535A9dcD7E32681".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            8453,
            Token::new(
                8453,
                "0x4200000000000000000000000000000000000006".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            56,
            Token::new(
                56,
                "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c".to_string(),
                18,
                Some("WBNB".to_string()),
                Some("Wrapped BNB".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            137,
            Token::new(
                137,
                "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270".to_string(),
                18,
                Some("WMATIC".to_string()),
                Some("Wrapped MATIC".to_string()),
                None,
                None,
            ),
        );
        tokens.insert(
            43114,
            Token::new(
                43114,
                "0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7".to_string(),
                18,
                Some("WAVAX".to_string()),
                Some("Wrapped AVAX".to_string()),
                None,
                None,
            ),
        );

        Self { tokens }
    }

    pub fn get(&self, chain_id: u32) -> Option<&Token> {
        self.tokens.get(&chain_id)
    }
}
