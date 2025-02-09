#![allow(non_camel_case_types)]

/// Represents the unique identifier for different blockchain networks supported by the Uniswap SDK.
///
/// Each variant corresponds to a specific blockchain network, identified by its unique chain ID.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ChainId {
    MAINNET = 1,
    GOERLI = 5,
    SEPOLIA = 11155111,
    OPTIMISM = 10,
    OPTIMISM_GOERLI = 420,
    OPTIMISM_SEPOLIA = 11155420,
    ARBITRUM_ONE = 42161,
    ARBITRUM_GOERLI = 421613,
    ARBITRUM_SEPOLIA = 421614,
    POLYGON = 137,
    POLYGON_MUMBAI = 80001,
    CELO = 42220,
    CELO_ALFAJORES = 44787,
    GNOSIS = 100,
    MOONBEAM = 1284,
    BNB = 56,
    AVALANCHE = 43114,
    BASE_GOERLI = 84531,
    BASE_SEPOLIA = 84532,
    BASE = 8453,
    ZORA = 7777777,
    ZORA_SEPOLIA = 999999999,
    ROOTSTOCK = 30,
    BLAST = 81457,
    ZKSYNC = 324,
    WORLDCHAIN = 480,
    UNICHAIN_SEPOLIA = 1301,
    UNICHAIN = 130,
    MONAD_TESTNET = 10143,
}

/// A list of `ChainId` constants representing the blockchain networks supported by the Uniswap SDK.
///
/// This array includes all the `ChainId` variants that are supported by the SDK, making it easy to
/// iterate over or check for supported chains.
pub const SUPPORTED_CHAINS: [ChainId; 27] = [
    ChainId::MAINNET,
    ChainId::OPTIMISM,
    ChainId::OPTIMISM_GOERLI,
    ChainId::OPTIMISM_SEPOLIA,
    ChainId::ARBITRUM_ONE,
    ChainId::ARBITRUM_GOERLI,
    ChainId::ARBITRUM_SEPOLIA,
    ChainId::POLYGON,
    ChainId::POLYGON_MUMBAI,
    ChainId::GOERLI,
    ChainId::SEPOLIA,
    ChainId::CELO_ALFAJORES,
    ChainId::CELO,
    ChainId::BNB,
    ChainId::AVALANCHE,
    ChainId::BASE,
    ChainId::BASE_GOERLI,
    ChainId::BASE_SEPOLIA,
    ChainId::ZORA,
    ChainId::ZORA_SEPOLIA,
    ChainId::ROOTSTOCK,
    ChainId::BLAST,
    ChainId::ZKSYNC,
    ChainId::WORLDCHAIN,
    ChainId::UNICHAIN_SEPOLIA,
    ChainId::UNICHAIN,
    ChainId::MONAD_TESTNET,
];
