use super::{base_currency::CurrencyLike, currency::CurrencyTrait};
use alloy_primitives::Address;
use num_bigint::BigUint;

/// Represents an ERC20 token with a unique address and some metadata.
pub type Token = CurrencyLike<TokenMeta>;

#[derive(Clone, PartialEq)]
pub struct TokenMeta {
    pub address: Address,
    pub buy_fee_bps: Option<BigUint>,
    pub sell_fee_bps: Option<BigUint>,
}

impl CurrencyTrait for Token {
    fn is_native(&self) -> bool {
        false
    }

    fn address(&self) -> Address {
        self.meta.address
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
            false => self.chain_id == other.chain_id() && self.address() == other.address(),
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
        decimals: u8,
        symbol: Option<String>,
        name: Option<String>,
        buy_fee_bps: Option<BigUint>,
        sell_fee_bps: Option<BigUint>,
    ) -> Self {
        assert!(chain_id > 0, "CHAIN_ID");
        assert!(decimals < 255, "DECIMALS");
        Self {
            chain_id,
            decimals,
            symbol,
            name,
            meta: TokenMeta {
                address: address.parse().unwrap(),
                buy_fee_bps,
                sell_fee_bps,
            },
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
        assert_ne!(self.address(), other.address(), "ADDRESSES");
        self.address().lt(&other.address())
    }
}

#[cfg(test)]
mod tests {
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

        assert!(token.address().eq(&ADDRESS_ONE.parse::<Address>().unwrap()));
        assert!(token_1
            .address()
            .eq(&ADDRESS_TWO.parse::<Address>().unwrap()));
    }

    #[test]
    #[should_panic(expected = "DECIMALS")]
    fn test_expect_revert_overflow_dec() {
        let _token = Token::new(
            4,
            ADDRESS_ONE.to_string(),
            255,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );
    }

    #[test]
    fn test_false_if_diff_chain_id() {
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

        assert!(!token.equals(&token_1));
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

        assert!(token.equals(&token_1), "true even if names differ");
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

        assert!(token.equals(&token_1), "true even if symbols differ");
    }

    #[test]
    fn test_false_if_diff_address() {
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

        assert!(!token.equals(&token_1));
    }

    #[test]
    fn test_true_if_diff_decimals() {
        assert!(
            Token::new(1, ADDRESS_ONE.to_string(), 9, None, None, None, None,).equals(&Token::new(
                1,
                ADDRESS_ONE.to_string(),
                18,
                None,
                None,
                None,
                None,
            ))
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

        assert_eq!(token.equals(&token_1), token_1.equals(&token));
    }

    #[test]
    fn test_true_on_reference_equality() {
        let token = Token::new(
            1,
            ADDRESS_ONE.to_string(),
            18,
            Some("Test".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        assert!(token.equals(&token));
    }

    #[test]
    fn test_true_if_same_address() {
        let token = Token::new(
            1,
            ADDRESS_ONE.to_string(),
            9,
            Some("abc".to_string()),
            Some("def".to_string()),
            None,
            None,
        );
        let token_1 = Token::new(
            1,
            ADDRESS_ONE.to_string(),
            18,
            Some("ghi".to_string()),
            Some("jkl".to_string()),
            None,
            None,
        );

        assert!(token.equals(&token_1));
    }

    #[test]
    fn test_true_if_one_token_is_checksummed_and_the_other_is_not() {
        let token_a = Token::new(
            1,
            DAI_MAINNET.to_string(),
            18,
            Some("DAI".to_string()),
            None,
            None,
            None,
        );
        let token_b = Token::new(
            1,
            DAI_MAINNET.to_string().to_lowercase(),
            18,
            Some("DAI".to_string()),
            None,
            None,
            None,
        );

        assert!(token_a.equals(&token_b));
    }
}
