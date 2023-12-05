use super::{base_currency::BaseCurrency, token::Token};

#[derive(PartialEq)]
pub struct Ether {
    base_currency: BaseCurrency,
    wrapped: Token,
}

impl Ether {
    pub fn new(chain_id: u32) -> Self {
        Self {
            base_currency: BaseCurrency::new(
                chain_id,
                18,
                Some("Ether".to_string()),
                Some("ETH".to_string()),
            ),
            wrapped: Token::new(
                chain_id,
                "0x".to_string(),
                18,
                Some("WETH".to_string()),
                Some("Wrapped Ether".to_string()),
                None,
                None,
            ),
        }
    }

    pub fn wrapped(&self) -> &Token {
        &self.wrapped
    }

    pub fn on_chain() ->  Ether{
        Ether::new(1)
    }

    pub fn equals(&self, other: &BaseCurrency) -> bool {
        other.is_native && other.chain_id == self.base_currency.chain_id
    }
}

#[cfg(test)]
mod tests {
    use super::Ether;

    #[test]
    fn test_ethers() {

        let eth = Ether::new(1);

        assert!(eth == Ether::on_chain(), "not equal");
    }
    
    #[test]
    #[should_panic]
    fn test_expect_revert() {
        let eth = Ether::new(2);

        assert!(eth == Ether::on_chain(), "not equal");
    }

    #[test]
    fn test_wrapped() {
        let eth = Ether::new(1);
        let eth2 = Ether::new(1);
        let weth = Ether::wrapped(&eth);
        assert!(weth.to_owned() == eth2.wrapped, "NOT WETH");
    }


    #[test]
    #[should_panic]
    fn test_expect_revert_wrapped() {
        let eth = Ether::new(1);
        let eth2 = Ether::new(2);
        let weth = Ether::wrapped(&eth);
        assert!(weth.to_owned() == eth2.wrapped, "NOT WETH");
    }
}
