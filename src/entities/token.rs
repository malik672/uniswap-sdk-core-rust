use super::base_currency::BaseCurrency;
use num_bigint::BigUint;

#[derive(Clone, PartialEq)] 
pub struct Token {
    pub base_currency: BaseCurrency,
    pub address: String,
    //bypass_checksum: bool,
    pub buy_fee_bps: Option<BigUint>,
    pub sell_fee_bps: Option<BigUint>,
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
            base_currency: BaseCurrency::new(chain_id, decimals, symbol, name),
            address,
            buy_fee_bps,
            sell_fee_bps,
        }
    }

    pub fn is_native() -> bool {
        false
    }

    pub fn is_token() -> bool {
        true
    }

    pub fn equals(&self, other: &Token) -> bool {
        self.base_currency.chain_id == other.base_currency.chain_id
    }

    pub fn sorts_before(&self, other: &Token) -> bool {
        assert!(
            self.base_currency.chain_id == other.base_currency.chain_id,
            "CHAIN_IDS"
        );
        assert!(self.address != other.address, "ADDRESSES");
        self.address < other.address
    }

    pub fn wrapped(&self) -> Token {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    //should test for neg chain_id or neg decicals or neg_buyfee or neg sellfee, but the compiler will panic by itself, so no need
    use super::Token;
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

        assert!(token.address == ADDRESS_ONE.to_string());
        assert!(token_1.address == ADDRESS_TWO.to_string());
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
            token == token_1,
            "SHOULD_FAILS_EVEN_THOUGH_CHAIN_ID_IS_DIFFERENT"
        );
    }

    #[test]
    #[should_panic]
    fn test_expect_revert_diff_name() {
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
            token == token_1,
            "SHOULD_FAILS_EVEN_THOUGH_NAME_IS_DIFFERENT"
        );
    }

    #[test]
    #[should_panic]
    fn test_expect_revert_diff_symbol() {
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
            Some("WETest".to_string()),
            Some("Te".to_string()),
            None,
            None,
        );

        assert!(
            token == token_1,
            "SHOULD_FAILS_EVEN_THOUGH_SYMBOL_IS_DIFFERENT"
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
            token == token_1,
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

        assert!(token.equals(&token_1) == token_1.equals(&token), "SHOULD_FAILS_EVEN_THOUGH_ADDRESS_IS_DIFFERENT, SHOULD ONLY REVERT FOR DIFFERENT CHAIN_ID");
    }
}
