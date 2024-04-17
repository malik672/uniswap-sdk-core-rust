#[derive(Debug, Clone, Copy)]
/// Represents the unique identifier for different blockchain networks supported by the Uniswap SDK.
///
/// Each variant corresponds to a specific blockchain network, identified by its unique chain ID.
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
    OPTIMISMGOERLI = 420,
    /// The Optimism Sepolia Testnet.
    OPTIMISMSEPOLIA = 11155420,
    /// The Arbitrum One network.
    ARBITRUMONE = 42161,
    /// The Arbitrum Goerli Testnet.
    ARBITRUMGOERLI = 421613,
    /// The Arbitrum Sepolia Testnet.
    ARBITRUMSEPOLIA = 421614,
    /// The Polygon network.
    POLYGON = 137,
    /// The Polygon Mumbai Testnet.
    POLYGONMUMBAI = 80001,
    /// The Celo network.
    CELO = 42220,
    /// The Celo Alfajores Testnet.
    CELOALFAJORES = 44787,
    /// The Gnosis network.
    GNOSIS = 100,
    /// The Moonbeam network.
    MOONBEAM = 1284,
    /// The Binance Smart Chain (BSC).
    BNB = 56,
    /// The Avalanche network.
    AVALANCHE = 43114,
    /// The Base network.
    BASEGOERLI = 84531,
    /// The Base Goerli Testnet.
    BASE = 8453,
    /// The Zora network.
    ZORA = 7777777,
    /// The Zora Sepolia Testnet.
    ZORASEPOLIA = 999999999,
    /// The Rootstock network.
    ROOTSTOCK = 30,
}

/// A list of `ChainId` constants representing the blockchain networks supported by the Uniswap SDK.
///
/// This array includes all the `ChainId` variants that are supported by the SDK, making it easy to
/// iterate over or check for supported chains.
pub const SUPPORTED_CHAINS: [ChainId; 20] = [
    ChainId::MAINNET,
    ChainId::OPTIMISM,
    ChainId::OPTIMISMGOERLI,
    ChainId::OPTIMISMSEPOLIA,
    ChainId::ARBITRUMONE,
    ChainId::ARBITRUMGOERLI,
    ChainId::ARBITRUMSEPOLIA,
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
    ChainId::ZORA,
    ChainId::ZORASEPOLIA,
    ChainId::ROOTSTOCK,
];

#[derive(Debug, Clone, Copy)]
/// Represents the names of native currencies supported by the Uniswap SDK.
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
}
