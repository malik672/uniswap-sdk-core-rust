#![allow(non_camel_case_types)]

/// Represents the unique identifier for different blockchain networks supported by the Uniswap SDK.
///
/// Each variant corresponds to a specific blockchain network, identified by its unique chain ID.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ChainId {
    /// The Ethereum Mainnet.
    MAINNET = 1,
    /// The Goerli Testnet.
    GOERLI = 5,
    /// The Sepolia Testnet.
    SEPOLIA = 11155111,
    /// The Optimism network.
    OPTIMISM = 10,
    /// The Optimism Goerli Testnet.
    OPTIMISM_GOERLI = 420,
    /// The Optimism Sepolia Testnet.
    OPTIMISM_SEPOLIA = 11155420,
    /// The Arbitrum One network.
    ARBITRUM_ONE = 42161,
    /// The Arbitrum Goerli Testnet.
    ARBITRUM_GOERLI = 421613,
    /// The Arbitrum Sepolia Testnet.
    ARBITRUM_SEPOLIA = 421614,
    /// The Polygon network.
    POLYGON = 137,
    /// The Polygon Mumbai Testnet.
    POLYGON_MUMBAI = 80001,
    /// The Celo network.
    CELO = 42220,
    /// The Celo Alfajores Testnet.
    CELO_ALFAJORES = 44787,
    /// The Gnosis network.
    GNOSIS = 100,
    /// The Moonbeam network.
    MOONBEAM = 1284,
    /// The Binance Smart Chain (BSC).
    BNB = 56,
    /// The Avalanche network.
    AVALANCHE = 43114,
    /// The Base network.
    BASE_GOERLI = 84531,
    /// The Base Goerli Testnet.
    BASE = 8453,
    /// The Zora network.
    ZORA = 7777777,
    /// The Zora Sepolia Testnet.
    ZORA_SEPOLIA = 999999999,
    /// The Rootstock network.
    ROOTSTOCK = 30,
    /// The Blast network.
    BLAST = 81457,
}

/// A list of `ChainId` constants representing the blockchain networks supported by the Uniswap SDK.
///
/// This array includes all the `ChainId` variants that are supported by the SDK, making it easy to
/// iterate over or check for supported chains.
pub const SUPPORTED_CHAINS: [ChainId; 21] = [
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
    ChainId::ZORA,
    ChainId::ZORA_SEPOLIA,
    ChainId::ROOTSTOCK,
    ChainId::BLAST,
];

/// Represents the names of native currencies supported by the Uniswap SDK.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum NativeCurrencyName {
    /// Ethereum's native currency.
    ETHER,
    /// Polygon's native currency.
    MATIC,
    /// Celo's native currency.
    CELO,
    /// Gnosis's native currency.
    GNOSIS,
    /// Moonbeam's native currency.
    MOONBEAM,
    /// Binance Smart Chain's native currency.
    BNB,
    /// Avalanche's native currency.
    AVAX,
    /// Rootstock's native currency.
    ROOTSTOCK,
    /// Blast native currency.
    BLAST,
}
