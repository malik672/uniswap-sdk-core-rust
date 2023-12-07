use super::chains::ChainId;
use lazy_static::lazy_static;
use std::collections::HashMap;
type AddressMap = HashMap<u32, String>;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ChainAddresses {
    v3_core_factory_address: String,
    multicall_address: String,
    quoter_address: String,
    v3_migrator_address: Option<String>,
    nonfungible_position_manager_address: Option<String>,
    tick_lens_address: Option<String>,
    swap_router02_address: Option<String>,
    v1_mixed_route_quoter_address: Option<String>,
}

pub const DefaultNetworks: [ChainId; 3] = [ChainId::MAINNET, ChainId::GOERLI, ChainId::SEPOLIA];

pub fn construct_same_address_map(address: &str, additional_networks: Vec<u32>) -> AddressMap {
    let mut map = AddressMap::new();
    let default_networks = vec![1, 2, 3]; // Placeholder for actual default networks
    for &network in default_networks.iter().chain(additional_networks.iter()) {
        map.insert(network, address.to_string());
    }
    map
}

lazy_static! {
    #[derive(Copy, Clone)]
    pub static ref UNIADDRESSES: AddressMap = construct_same_address_map(
        "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
        [
            ChainId::OPTIMISM as u32,
            ChainId::ARBITRUMONE as u32,
            ChainId::POLYGON as u32,
            ChainId::POLYGONMUMBAI as u32,
            ChainId::SEPOLIA as u32,
        ]
        .to_vec()
    );
}

pub const UNISWAP_NFT_AIRDROP_CLAIM_ADDRESS: &str = "0x8B799381ac40b838BBA4131ffB26197C432AFe78";

pub const V2_FACTORY_ADDRESS: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";

lazy_static! {
    pub static ref V2_FACTORY_ADDRESSES: AddressMap = construct_same_address_map(
        V2_FACTORY_ADDRESS,
        [
            ChainId::POLYGON as u32,
            ChainId::OPTIMISM as u32,
            ChainId::CELO as u32,
            ChainId::ARBITRUMONE as u32,
            ChainId::BNB as u32,
            ChainId::AVALANCHE as u32,
            ChainId::BASE as u32,
        ]
        .to_vec()
    );
}

pub const V2_ROUTER_ADDRESS: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";

lazy_static! {
    pub static ref V2_ROUTER_ADDRESSES: AddressMap =
        construct_same_address_map(V2_ROUTER_ADDRESS, [].to_vec());
}

pub fn default_addr() -> ChainAddresses {
    // Networks that share most of the same addresses i.e. Mainnet, Goerli, Optimism, Arbitrum, Polygon
    let default_addresses: ChainAddresses = ChainAddresses {
        v3_core_factory_address: "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string(),
        multicall_address: "0x1F98415757620B543A52E61c46B32eB19261F984".to_string(),
        quoter_address: "0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6".to_string(),
        v3_migrator_address: Some("0xA5644E29708357803b5A882D272c41cC0dF92B34".to_string()),
        nonfungible_position_manager_address: Some(
            "0xC36442b4a4522E871399CD717aBDD847Ab11FE88".to_string(),
        ),
        tick_lens_address: None,
        swap_router02_address: None,
        v1_mixed_route_quoter_address: None,
    };
    default_addresses
}

pub fn mainnet_address() {
    let mut mainnet_addresses = default_addr().clone();
    mainnet_addresses.v1_mixed_route_quoter_address =
        Some("0x84E44095eeBfEC7793Cd7d5b57B7e401D7f1cA2E".to_string());
}
