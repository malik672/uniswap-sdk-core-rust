/// A currency is any fungible financial instrument, including Ether, all ERC20 tokens, and other chain-native currencies
#[derive(Clone, PartialEq)]
pub struct BaseCurrency {
    pub chain_id: u32,
    pub decimals: u32,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub is_native: bool,
}

impl BaseCurrency {
    pub fn new(chain_id: u32, decimals: u32, name: Option<String>, symbol: Option<String>) -> Self {
        assert!(chain_id > 0, "CHAIN_ID");
        assert!(decimals < 255, "DECIMALS");

        Self {
            chain_id,
            decimals,
            name,
            symbol,
            is_native: Self::is_native(),
        }
    }

    /// Returns whether the currency is native to the chain and must be wrapped (e.g. Ether)
    pub fn is_native() -> bool {
        true
    }
}
