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

pub const SUPPORTED_CHAINS: [ChainId; 15] = [
   ChainId::MAINNET,
   ChainId::OPTIMISM,
   ChainId::OPTIMISMGOERLI,
   ChainId::ARBITRUMONE,
   ChainId::ARBITRUMGOERLI,
   ChainId::POLYGON,
   ChainId::POLYGONMUMBAI,
   ChainId::GOERLI,
   ChainId::SEPOLIA,
   ChainId::CELOALFAJORES,
   ChainId::CELO,
   ChainId::BNB,
   ChainId::AVALANCHE,
   ChainId::BASE,
   ChainId::BASEGOERLI,
];


pub enum NativeCurrencyName {
    ETHER,
    MATIC,
    CELO ,
    GNOSIS,
    MOONBEAM,
    BNB,
    AVAX,
}
