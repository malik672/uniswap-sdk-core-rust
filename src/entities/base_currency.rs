/**
 * A currency is any fungible financial instrument, including Ether, all ERC20 tokens, and other chain-native currencies
 */
pub struct BaseCurrency {
    chain_id: u32,
    decimals: u32,
    name: Option<str>,
    symbol: Option<str>,
}

impl BaseCurrency {
    pub fn new(
        chain_id: u32,
        decimals: u32,
        name: Option<str>,
        symbol: Option<str>,
    ) -> Self {
        assert!(chain_id > 0, "CHAIN_ID");
        assert!(decimals >= 0 && decimals < 255, "DECIMALS");

        Self {
            chain_id,
            decimals,
            name,
            symbol,
        }
    }
}

/**
 * Returns whether the currency is native to the chain and must be wrapped (e.g. Ether)
*/
pub fn isNative() -> bool {

} 