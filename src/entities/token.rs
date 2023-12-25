use super::{base_currency::BaseCurrency, currency::CurrencyTrait};
use num_bigint::BigUint;

/// Represents an ERC20 token with a unique address and some metadata.
#[derive(Clone, PartialEq)]
pub struct Token {
    pub chain_id: u32,
    pub address: String,
    pub decimals: u32,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub buy_fee_bps: Option<BigUint>,
    pub sell_fee_bps: Option<BigUint>,
}

impl BaseCurrency for Token {
    fn chain_id(&self) -> u32 {
        self.chain_id
    }

    fn decimals(&self) -> u32 {
        self.decimals
    }

    fn symbol(&self) -> Option<String> {
        self.symbol.clone()
    }

    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Returns true if the two tokens are equivalent, i.e. have the same chainId and address.
    ///
    /// # Arguments
    ///
    /// * `other`: other token to compare
    ///
    /// returns: bool
    ///
    fn equals(&self, other: &impl CurrencyTrait) -> bool {
        match other.is_native() {
            false => {
                self.chain_id == other.chain_id()
                    && self.address.to_lowercase() == other.address().to_lowercase()
            }
            _ => false,
        }
    }

    /// Return this token, which does not need to be wrapped
    fn wrapped(&self) -> Token {
        self.clone()
    }
}

impl Token {
    pub fn new(
        chain_id: u32,
        address: String,
        decimals: u32,
        symbol: Option<String>,
        name: Option<String>,
        buy_fee_bps: Option<BigUint>,
        sell_fee_bps: Option<BigUint>,
    ) -> Self {
        assert!(chain_id > 0, "CHAIN_ID");
        assert!(decimals < 255, "DECIMALS");
        Self {
            chain_id,
            address,
            decimals,
            symbol,
            name,
            buy_fee_bps,
            sell_fee_bps,
        }
    }

    /// Returns true if the address of this token sorts before the address of the other token.
    /// Panics if the tokens have the same address or if the tokens are on different chains.
    ///
    /// # Arguments
    ///
    /// * `other`: other token to compare
    ///
    pub fn sorts_before(&self, other: &Token) -> bool {
        assert_eq!(self.chain_id, other.chain_id, "CHAIN_IDS");
        assert_ne!(
            self.address.to_lowercase(),
            other.address.to_lowercase(),
            "ADDRESSES"
        );
        self.address.to_lowercase() < other.address.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::currency::Currency;
    //should test for neg chain_id or neg decimals or neg buy_fee or neg sell_fee, but the compiler will panic by itself, so no need
    use super::*;

    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";
    const ADDRESS_TWO: &str = "0x0000000000000000000000000000000000000002";
    const DAI_MAINNET: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

    #[test]
    fn test_token() {
        let token = Token::new(
            2,
            ADDRESS_ONE.to_string(),
            18,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );
        let token_1 = Token::new(
            2,
            ADDRESS_TWO.to_string(),
            18,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        assert_eq!(token.address, *ADDRESS_ONE);
        assert_eq!(token_1.address, *ADDRESS_TWO);
    }

    #[test]
    #[should_panic]
    fn test_expect_revert_overflow_dec() {
        let _token = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            256,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );
    }

    #[test]
    #[should_panic]
    fn test_expect_revert_diff_chain_id() {
        let token = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            25,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        let token_1 = Token::new(
            3,
            ADDRESS_ONE.to_string(),
            25,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        assert!(
            token.equals(&Currency::Token(token_1)),
            "SHOULD_FAILS_EVEN_THOUGH_CHAIN_ID_IS_DIFFERENT"
        );
    }

    #[test]
    fn test_diff_name() {
        let token = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            25,
            Some("Test".to_string()),
            Some("TeW".to_string()),
            None,
            None,
        );

        let token_1 = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            25,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        assert!(
            token.equals(&Currency::Token(token_1)),
            "true even if names differ"
        );
    }

    #[test]
    fn test_diff_symbol() {
        let token = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            25,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        let token_1 = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            25,
            Some("WETest".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        assert!(
            token.equals(&Currency::Token(token_1)),
            "true even if symbols differ"
        );
    }

    #[test]
    #[should_panic]
    fn test_expect_revert_diff_address() {
        let token = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            25,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        let token_1 = Token::new(
            4,
            DAI_MAINNET.to_string(),
            25,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        assert!(
            token.equals(&Currency::Token(token_1)),
            "SHOULD_FAILS_EVEN_THOUGH_ADDRESS_IS_DIFFERENT"
        );
    }

    #[test]
    fn test_assert_both_tokens() {
        let token = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            25,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        let token_1 = Token::new(
            4,
            DAI_MAINNET.to_string(),
            25,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        assert_eq!(
            token.equals(&Currency::Token(token_1.clone())),
            token_1.equals(&Currency::Token(token)),
            "SHOULD_FAILS_EVEN_THOUGH_ADDRESS_IS_DIFFERENT, SHOULD ONLY REVERT FOR DIFFERENT CHAIN_ID"
        );
    }
}
