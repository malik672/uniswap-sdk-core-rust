#[derive(Copy, Clone)]
pub enum ChainId {
    MAINNET = 1,
    GOERLI = 5,
    SEPOLIA = 11155111,
    OPTIMISM = 10,
    OPTIMISMGOERLI = 420,
    ARBITRUMONE = 42161,
    ARBITRUMGOERLI = 421613,
    POLYGON = 137,
    POLYGONMUMBAI = 80001,
    CELO = 42220,
    CELOALFAJORES = 44787,
    GNOSIS = 100,
    MOONBEAM = 1284,
    BNB = 56,
    AVALANCHE = 43114,
    BASEGOERLI = 84531,
    BASE = 8453,
}

pub enum NativeCurrencyName {
    ETHER,
    MATIC,
    CELO ,
    GNOSIS,
    MOONBEAM,
    BNB,
    AVAX,
}
