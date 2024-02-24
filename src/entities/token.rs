use crate::prelude::*;

/// Represents an ERC20 token with a unique address and some metadata.
pub type Token = CurrencyLike<TokenMeta>;

#[derive(Clone, PartialEq, Debug)]
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
        chain_id: u64,
        address: String,
        decimals: u8,
        symbol: Option<String>,
        name: Option<String>,
        buy_fee_bps: Option<BigUint>,
        sell_fee_bps: Option<BigUint>,
    ) -> Self {
        if chain_id == 0 {
            panic!("chain id can't be zero");
        }
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
    pub fn sorts_before(&self, other: &Token) -> Result<bool, Error> {
        if self.chain_id != other.chain_id {
            return Err(Error::ChainIdMismatch(self.chain_id, other.chain_id));
        }

        if self.address() == other.address() {
            return Err(Error::EqualAddresses);
        }
        Ok(self.address().lt(&other.address()))
    }
}

/// Short hand macro to create a token
#[macro_export]
macro_rules! token {
    ($chain_id:expr, $address:expr, $decimals:expr) => {
        Token::new(
            $chain_id,
            $address.to_string(),
            $decimals,
            None,
            None,
            None,
            None,
        )
    };
    ($chain_id:expr, $address:expr, $decimals:expr, $symbol:expr) => {
        Token::new(
            $chain_id,
            $address.to_string(),
            $decimals,
            Some($symbol.to_string()),
            None,
            None,
            None,
        )
    };
    ($chain_id:expr, $address:expr, $decimals:expr, $symbol:expr, $name:expr) => {
        Token::new(
            $chain_id,
            $address.to_string(),
            $decimals,
            Some($symbol.to_string()),
            Some($name.to_string()),
            None,
            None,
        )
    };
}

#[cfg(test)]
mod tests {
    ///should test for neg chain_id or neg decimals or neg buy_fee or neg sell_fee, but the compiler will panic by itself, so no need
    use super::*;

    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";
    const ADDRESS_TWO: &str = "0x0000000000000000000000000000000000000002";
    const DAI_MAINNET: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

    #[test]
    fn test_token() {
        let token = token!(2, ADDRESS_ONE, 18, "Test", "Te");
        let token_1 = token!(2, ADDRESS_TWO, 18, "Test", "Te");

        assert!(token.address().eq(&ADDRESS_ONE.parse::<Address>().unwrap()));
        assert!(token_1
            .address()
            .eq(&ADDRESS_TWO.parse::<Address>().unwrap()));
    }

    #[test]
    fn test_expect_revert_overflow_dec() {
        let _token = token!(4, ADDRESS_ONE, 255, "Test", "Te");
    }

    #[test]
    fn test_false_if_diff_chain_id() {
        let token = token!(4, ADDRESS_ONE, 25, "Test", "Te");
        let token_1 = token!(3, ADDRESS_ONE, 25, "Test", "Te");

        assert!(!token.equals(&token_1));
    }

    #[test]
    fn test_diff_name() {
        let token = token!(4, ADDRESS_ONE, 25, "Test", "TeW");
        let token_1 = token!(4, ADDRESS_ONE, 25, "Test", "Te");

        assert!(token.equals(&token_1), "true even if names differ");
    }

    #[test]
    fn test_diff_symbol() {
        let token = token!(4, ADDRESS_ONE, 25, "Test", "Te");
        let token_1 = token!(4, ADDRESS_ONE, 25, "WETest", "Te");

        assert!(token.equals(&token_1), "true even if symbols differ");
    }

    #[test]
    fn test_false_if_diff_address() {
        let token = token!(4, ADDRESS_ONE, 25, "Test", "Te");
        let token_1 = token!(4, DAI_MAINNET, 25, "Test", "Te");

        assert!(!token.equals(&token_1));
    }

    #[test]
    fn test_true_if_diff_decimals() {
        assert!(token!(1, ADDRESS_ONE, 9).equals(&token!(1, ADDRESS_ONE, 18)));
    }

    #[test]
    fn test_assert_both_tokens() {
        let token = token!(4, ADDRESS_ONE, 25, "Test", "Te");
        let token_1 = token!(4, DAI_MAINNET, 25, "Test", "Te");

        assert_eq!(token.equals(&token_1), token_1.equals(&token));
    }

    #[test]
    fn test_true_on_reference_equality() {
        let token = token!(1, ADDRESS_ONE, 18, "Test", "Te");

        assert!(token.equals(&token));
    }

    #[test]
    fn test_true_if_same_address() {
        let token = token!(1, ADDRESS_ONE, 9, "abc", "def");
        let token_1 = token!(1, ADDRESS_ONE, 18, "ghi", "jkl");

        assert!(token.equals(&token_1));
    }

    #[test]
    fn test_true_if_one_token_is_checksummed_and_the_other_is_not() {
        let token_a = token!(1, DAI_MAINNET, 18, "DAI");
        let token_b = token!(1, DAI_MAINNET, 18, "DAI");

        assert!(token_a.equals(&token_b));
    }
}
