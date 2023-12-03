use base_currency::BaseCurrency;
use num_bigint::BigInt;

pub struct Token {
    base_currency: BaseCurrency,
    bypass_checksum: bool,
    buy_fee_bps: BigNumber,
    sell_fee_bps: BigNumber,
}

impl Token {
    pub fn new(
        chain_id: u32,
        address: str,
        decimals: u32,
        symbol: Option<String>,
        name: Option<String>,
        buy_fee_bps: Option<BigInt>,
        sell_fee_bps: Option<BigInt>,
    ) -> Self {
        assert!(chain_id > 0, "CHAIN_ID");
        assert!(decimals >= 0 && decimals < 255, "DECIMALS");
        assert!(
            buy_fee_bps.unwrap_or(BigInt::from(0)) >= BigInt::from(0),
            "NON-NEGATIVE FOT FEES"
        );
        assert!(
            sell_fee_bps.unwrap_or(BigInt::from(0)) >= BigInt::from(0),
            "NON-NEGATIVE FOT FEES"
        );

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
