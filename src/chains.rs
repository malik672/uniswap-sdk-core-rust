#![allow(non_camel_case_types)]

use crate::{error::Error, prelude::HashMap};
use lazy_static::lazy_static;

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
    SONEIUM = 1868,
    MONAD = 143,
    XLAYER = 196,
    LINEA = 59144,
    TEMPO = 4217,
    MEGAETH = 4326,
    ARC = 5042,
    ROBINHOOD = 4663,
    INK = 57073,
}

lazy_static! {
    /// Average block time in seconds for chains with known deployment metadata.
    pub static ref AVERAGE_BLOCK_TIMES_SECONDS: HashMap<u64, f64> = HashMap::from_iter([
        (ChainId::MAINNET as u64, 12.0),
        (ChainId::OPTIMISM as u64, 2.0),
        (ChainId::ARBITRUM_ONE as u64, 0.25),
        (ChainId::POLYGON as u64, 1.75),
        (ChainId::CELO as u64, 1.0),
        (ChainId::BNB as u64, 0.45),
        (ChainId::AVALANCHE as u64, 1.0),
        (ChainId::BASE as u64, 2.0),
        (ChainId::ZORA as u64, 2.0),
        (ChainId::BLAST as u64, 2.0),
        (ChainId::WORLDCHAIN as u64, 2.0),
        (ChainId::UNICHAIN as u64, 1.0),
        (ChainId::SONEIUM as u64, 2.0),
        (ChainId::MONAD as u64, 0.4),
        (ChainId::XLAYER as u64, 1.0),
        (ChainId::TEMPO as u64, 0.5),
        (ChainId::MEGAETH as u64, 1.0),
        (ChainId::ARC as u64, 0.48),
        (ChainId::ROBINHOOD as u64, 0.1),
        (ChainId::INK as u64, 1.0),
    ]);
}

/// Returns the average block time in seconds for a chain.
///
/// # Errors
///
/// Returns [`Error::UnsupportedChain`] when the chain has no registered block time.
#[inline]
pub fn get_average_block_time_secs(chain_id: u64) -> Result<f64, Error> {
    AVERAGE_BLOCK_TIMES_SECONDS
        .get(&chain_id)
        .copied()
        .ok_or(Error::UnsupportedChain(chain_id))
}

/// Converts a wall-clock duration in seconds to a block count, rounding up.
///
/// # Errors
///
/// Returns [`Error::UnsupportedChain`] when the chain has no registered block time.
#[inline]
pub fn seconds_to_blocks(seconds: f64, chain_id: u64) -> Result<u64, Error> {
    Ok((seconds / get_average_block_time_secs(chain_id)?).ceil() as u64)
}

/// A list of `ChainId` constants representing the blockchain networks supported by the Uniswap SDK.
///
/// This array includes all the `ChainId` variants that are supported by the SDK, making it easy to
/// iterate over or check for supported chains.
pub const SUPPORTED_CHAINS: [ChainId; 36] = [
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
    ChainId::SONEIUM,
    ChainId::MONAD,
    ChainId::XLAYER,
    ChainId::LINEA,
    ChainId::TEMPO,
    ChainId::MEGAETH,
    ChainId::ARC,
    ChainId::ROBINHOOD,
    ChainId::INK,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_registered_average_block_times() {
        assert_eq!(get_average_block_time_secs(1), Ok(12.0));
        assert_eq!(get_average_block_time_secs(42161), Ok(0.25));
        assert_eq!(get_average_block_time_secs(4663), Ok(0.1));
        assert_eq!(get_average_block_time_secs(4326), Ok(1.0));
        assert_eq!(get_average_block_time_secs(5042), Ok(0.48));
        assert_eq!(get_average_block_time_secs(57073), Ok(1.0));
    }

    #[test]
    fn rejects_unregistered_chain() {
        assert_eq!(
            get_average_block_time_secs(99999),
            Err(Error::UnsupportedChain(99999))
        );
        assert_eq!(
            seconds_to_blocks(10.0, 99999),
            Err(Error::UnsupportedChain(99999))
        );
    }

    #[test]
    fn converts_seconds_to_blocks_with_ceil() {
        assert_eq!(seconds_to_blocks(8.0, 1), Ok(1));
        assert_eq!(seconds_to_blocks(8.0, 42161), Ok(32));
        assert_eq!(seconds_to_blocks(8.0, 4217), Ok(16));
        assert_eq!(seconds_to_blocks(8.0, 4326), Ok(8));
        assert_eq!(seconds_to_blocks(8.0, 5042), Ok(17));
        assert_eq!(seconds_to_blocks(8.0, 4663), Ok(80));
        assert_eq!(seconds_to_blocks(8.0, 57073), Ok(8));
        assert_eq!(seconds_to_blocks(1.0, 1), Ok(1));
    }
}
