use crate::entities::base_currency::BaseCurrency;
use crate::entities::token::Token;

pub struct Ether {
    base_currency: BaseCurrency,
    wrapped: Token,
}
  
  impl Ether {
    pub fn new(chain_id: u32) -> Self {
        Self {
            base_currency: BaseCurrency::new(chain_id, 18, Some("ETH".to_string()), Some("Ether".to_string())),
            wrapped: Token::new(chain_id, "0x".to_string(), 18, Some("WETH".to_string()), Some("Wrapped Ether".to_string()), None, None),
        }
    }
  
    pub fn wrapped(&self) -> &Token {
        &self.wrapped
    }
  
    pub fn equals(&self, other: &BaseCurrency) -> bool {
        other.is_native && other.chain_id == self.base_currency.chain_id
    }
  }
  