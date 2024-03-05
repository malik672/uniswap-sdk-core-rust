use crate::prelude::*;

type AddressMap = HashMap<u64, Address>;
type ChainMap = HashMap<u64, ChainAddresses>;
type ChainAddress = HashMap<u64, Address>;

#[derive(Clone, Copy)]
pub struct ChainAddresses {
    v3_core_factory_address: Address,
    multicall_address: Address,
    quoter_address: Address,
    v3_migrator_address: Option<Address>,
    nonfungible_position_manager_address: Option<Address>,
    tick_lens_address: Option<Address>,
    swap_router02_address: Option<Address>,
    v1_mixed_route_quoter_address: Option<Address>,
}

pub const DEFAULT_NETWORKS: [ChainId; 3] = [ChainId::MAINNET, ChainId::GOERLI, ChainId::SEPOLIA];

/// returns a hashmap of key pair input of chainid to address
///
/// # Arguments
///
/// * `address`: Address
/// * `additional networks`: a vector of chain ids
///
///
/// returns: [`AdresssMap`]
pub fn construct_same_address_map(address: Address, additional_networks: &[ChainId]) -> AddressMap {
    let mut networks = DEFAULT_NETWORKS.to_vec();
    networks.extend_from_slice(additional_networks);
    let mut map = AddressMap::new();
    for chain_id in networks {
        map.insert(chain_id as u64, address);
    }
    map
}

lazy_static! {
    #[derive(Debug, Clone, Copy)]
    pub static ref UNI_ADDRESSES: AddressMap = construct_same_address_map(
        address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984"),
        &[
            ChainId::OPTIMISM,
            ChainId::ARBITRUMONE,
            ChainId::POLYGON,
            ChainId::POLYGONMUMBAI,
            ChainId::SEPOLIA,
        ]
    );
}

pub const UNISWAP_NFT_AIRDROP_CLAIM_ADDRESS: Address =
    address!("8B799381ac40b838BBA4131ffB26197C432AFe78");

pub const V2_FACTORY_ADDRESS: Address = address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");

lazy_static! {
    pub static ref V2_FACTORY_ADDRESSES: HashMap<u64, Address> = {
        let mut m = HashMap::new();
        m.insert(ChainId::MAINNET as u64, V2_FACTORY_ADDRESS);
        m.insert(ChainId::GOERLI as u64, V2_FACTORY_ADDRESS);
        m.insert(
            ChainId::SEPOLIA as u64,
            address!("B7f907f7A9eBC822a80BD25E224be42Ce0A698A0"),
        );
        m.insert(
            ChainId::OPTIMISM as u64,
            address!("0c3c1c532F1e39EdF36BE9Fe0bE1410313E074Bf"),
        );
        m.insert(
            ChainId::ARBITRUMONE as u64,
            address!("f1D7CC64Fb4452F05c498126312eBE29f30Fbcf9"),
        );
        m.insert(
            ChainId::AVALANCHE as u64,
            address!("9e5A52f57b3038F1B8EeE45F28b3C1967e22799C"),
        );
        m.insert(
            ChainId::BASE as u64,
            address!("8909dc15e40173ff4699343b6eb8132c65e18ec6"),
        );
        m.insert(
            ChainId::BNB as u64,
            address!("8909Dc15e40173Ff4699343b6eB8132c65e18eC6"),
        );
        m.insert(
            ChainId::POLYGON as u64,
            address!("9e5A52f57b3038F1B8EeE45F28b3C1967e22799C"),
        );
        m.insert(
            ChainId::CELO as u64,
            address!("79a530c8e2fA8748B7B40dd3629C0520c2cCf03f"),
        );
        m
    };
}

pub const V2_ROUTER_ADDRESS: Address = address!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D");

lazy_static! {
    pub static ref V2_ROUTER_ADDRESSES: HashMap<u64, Address> = {
        let mut m = HashMap::new();
        m.insert(ChainId::MAINNET as u64, V2_ROUTER_ADDRESS);
        m.insert(ChainId::GOERLI as u64, V2_ROUTER_ADDRESS);
        m.insert(
            ChainId::ARBITRUMONE as u64,
            address!("4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
        );
        m.insert(
            ChainId::OPTIMISM as u64,
            address!("4a7b5da61326a6379179b40d00f57e5bbdc962c2"),
        );
        m.insert(
            ChainId::BASE as u64,
            address!("4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
        );
        m.insert(
            ChainId::AVALANCHE as u64,
            address!("4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
        );
        m.insert(
            ChainId::BNB as u64,
            address!("4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
        );
        m.insert(
            ChainId::POLYGON as u64,
            address!("edf6066a2b290c185783862c7f4776a2c8077ad1"),
        );
        m
    };
}

impl Default for ChainAddresses {
    /// Networks that share most of the same addresses i.e. Mainnet, Goerli, Optimism, Arbitrum,
    /// Polygon
    fn default() -> Self {
        Self {
            v3_core_factory_address: address!("1F98431c8aD98523631AE4a59f267346ea31F984"),
            multicall_address: address!("1F98415757620B543A52E61c46B32eB19261F984"),
            quoter_address: address!("b27308f9F90D607463bb33eA1BeBb41C27CE5AB6"),
            v3_migrator_address: Some(address!("A5644E29708357803b5A882D272c41cC0dF92B34")),
            nonfungible_position_manager_address: Some(address!(
                "C36442b4a4522E871399CD717aBDD847Ab11FE88"
            )),
            tick_lens_address: None,
            swap_router02_address: None,
            v1_mixed_route_quoter_address: None,
        }
    }
}

lazy_static! {
    pub static ref MAINNET_ADDRESSES: ChainAddresses = {
        ChainAddresses {
            v1_mixed_route_quoter_address: Some(address!(
                "84E44095eeBfEC7793Cd7d5b57B7e401D7f1cA2E"
            )),
            ..Default::default()
        }
    };
}

lazy_static! {
    pub static ref GOERLI_ADDRESSES: ChainAddresses = {
        ChainAddresses {
            v1_mixed_route_quoter_address: Some(address!(
                "Ba60b6e6fF25488308789E6e0A65D838be34194e"
            )),
            ..Default::default()
        }
    };
}

lazy_static! {
    pub static ref OPTIMISM_ADDRESSES: ChainAddresses = ChainAddresses::default();
}

lazy_static! {
    pub static ref ARBITUM_ONE_ADDRESSES: ChainAddresses = {
        ChainAddresses {
            multicall_address: address!("adF885960B47eA2CD9B55E6DAc6B42b7Cb2806dB"),
            tick_lens_address: Some(address!("bfd8137f7d1516D3ea5cA83523914859ec47F573")),
            ..Default::default()
        }
    };
}
lazy_static! {
    pub static ref POLYGON_ADDRESSES: ChainAddresses = ChainAddresses::default();
}

/// celo v3 addresses
pub const CELO_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("AfE208a311B21f13EF87E33A90049fC17A7acDEc"),
    multicall_address: address!("633987602DE5C4F337e3DbF265303A1080324204"),
    quoter_address: address!("82825d0554fA07f7FC52Ab63c961F330fdEFa8E8"),
    v3_migrator_address: Some(address!("3cFd4d48EDfDCC53D3f173F596f621064614C582")),
    nonfungible_position_manager_address: Some(address!(
        "3d79EdAaBC0EaB6F08ED885C05Fc0B014290D95A"
    )),
    tick_lens_address: Some(address!("5f115D9113F88e0a0Db1b5033D90D4a9690AcD3D")),
    swap_router02_address: None,
    v1_mixed_route_quoter_address: None,
};

/// BNB v3 addresses
pub const BNB_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("dB1d10011AD0Ff90774D0C6Bb92e5C5c8b4461F7"),
    multicall_address: address!("963Df249eD09c358A4819E39d9Cd5736c3087184"),
    quoter_address: address!("78D78E420Da98ad378D7799bE8f4AF69033EB077"),
    v3_migrator_address: Some(address!("32681814957e0C13117ddc0c2aba232b5c9e760f")),
    nonfungible_position_manager_address: Some(address!(
        "7b8A01B39D58278b5DE7e48c8449c9f4F5170613"
    )),
    tick_lens_address: Some(address!("D9270014D396281579760619CCf4c3af0501A47C")),
    swap_router02_address: Some(address!("B971eF87ede563556b2ED4b1C0b0019111Dd85d2")),
    v1_mixed_route_quoter_address: None,
};

/// Optimism Goerli addresses
pub const OPTIMISM_GOERLI_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("B656dA17129e7EB733A557f4EBc57B76CFbB5d10"),
    multicall_address: address!("07F2D8a2a02251B62af965f22fC4744A5f96BCCd"),
    quoter_address: address!("9569CbA925c8ca2248772A9A4976A516743A246F"),
    v3_migrator_address: Some(address!("f6c55fBe84B1C8c3283533c53F51bC32F5C7Aba8")),
    nonfungible_position_manager_address: Some(address!(
        "39Ca85Af2F383190cBf7d7c41ED9202D27426EF6"
    )),
    tick_lens_address: Some(address!("e6140Bd164b63E8BfCfc40D5dF952f83e171758e")),
    swap_router02_address: None,
    v1_mixed_route_quoter_address: None,
};

/// Optimism Sepolia addresses
pub const OPTIMISM_SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("8CE191193D15ea94e11d327b4c7ad8bbE520f6aF"),
    multicall_address: address!("80e4e06841bb76AA9735E0448cB8d003C0EF009a"),
    quoter_address: address!("0FBEa6cf957d95ee9313490050F6A0DA68039404"),
    v3_migrator_address: Some(address!("E7EcbAAaA54D007A00dbb6c1d2f150066D69dA07")),
    nonfungible_position_manager_address: Some(address!(
        "dA75cEf1C93078e8b736FCA5D5a30adb97C8957d"
    )),
    tick_lens_address: Some(address!("Cb7f54747F58F8944973cea5b8f4ac2209BadDC5")),
    swap_router02_address: Some(address!("94cC0AaC535CCDB3C01d6787D6413C739ae12bc4")),
    v1_mixed_route_quoter_address: None,
};

/// Arbitrum Goerli v3 addresses
pub const ARBITRUM_GOERLI_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("4893376342d5D7b3e31d4184c08b265e5aB2A3f6"),
    multicall_address: address!("8260CB40247290317a4c062F3542622367F206Ee"),
    quoter_address: address!("1dd92b83591781D0C6d98d07391eea4b9a6008FA"),
    v3_migrator_address: Some(address!("A815919D2584Ac3F76ea9CB62E6Fd40a43BCe0C3")),
    nonfungible_position_manager_address: Some(address!(
        "622e4726a167799826d1E1D150b076A7725f5D81"
    )),
    tick_lens_address: Some(address!("b52429333da969a0C79a60930a4Bf0020E5D1DE8")),
    swap_router02_address: None,
    v1_mixed_route_quoter_address: None,
};

/// Arbitrum sepolia v3 addresses
pub const ARBITRUM_SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("248AB79Bbb9bC29bB72f7Cd42F17e054Fc40188e"),
    multicall_address: address!("2B718b475e385eD29F56775a66aAB1F5cC6B2A0A"),
    quoter_address: address!("2779a0CC1c3e0E44D2542EC3e79e3864Ae93Ef0B"),
    v3_migrator_address: Some(address!("398f43ef2c67B941147157DA1c5a868E906E043D")),
    nonfungible_position_manager_address: Some(address!(
        "6b2937Bde17889EDCf8fbD8dE31C3C2a70Bc4d65"
    )),
    tick_lens_address: Some(address!("b52429333da969a0C79a60930a4Bf0020E5D1DE8")),
    swap_router02_address: Some(address!("101F443B4d1b059569D643917553c771E1b9663E")),
    v1_mixed_route_quoter_address: None,
};

/// sepolia v3 addresses
pub const SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("0227628f3F023bb0B980b67D528571c95c6DaC1c"),
    multicall_address: address!("D7F33bCdb21b359c8ee6F0251d30E94832baAd07"),
    quoter_address: address!("Ed1f6473345F45b75F8179591dd5bA1888cf2FB3"),
    v3_migrator_address: Some(address!("729004182cF005CEC8Bd85df140094b6aCbe8b15")),
    nonfungible_position_manager_address: Some(address!(
        "1238536071E1c677A632429e3655c799b22cDA52"
    )),
    tick_lens_address: Some(address!("0b343475d44EC2b4b8243EBF81dc888BF0A14b36")),
    swap_router02_address: Some(address!("3bFA4769FB09eefC5a80d6E87c3B9C650f7Ae48E")),
    v1_mixed_route_quoter_address: None,
};

/// Avalanche v3 addresses
pub const AVALANCHE_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("740b1c1de25031C31FF4fC9A62f554A55cdC1baD"),
    multicall_address: address!("0139141Cd4Ee88dF3Cdb65881D411bAE271Ef0C2"),
    quoter_address: address!("be0F5544EC67e9B3b2D979aaA43f18Fd87E6257F"),
    v3_migrator_address: Some(address!("44f5f1f5E452ea8d29C890E8F6e893fC0f1f0f97")),
    nonfungible_position_manager_address: Some(address!(
        "655C406EBFa14EE2006250925e54ec43AD184f8B"
    )),
    tick_lens_address: Some(address!("EB9fFC8bf81b4fFd11fb6A63a6B0f098c6e21950")),
    swap_router02_address: Some(address!("bb00FF08d01D300023C629E8fFfFcb65A5a578cE")),
    v1_mixed_route_quoter_address: None,
};

/// Base v3 addresses
pub const BASE_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("33128a8fC17869897dcE68Ed026d694621f6FDfD"),
    multicall_address: address!("091e99cb1C49331a94dD62755D168E941AbD0693"),
    quoter_address: address!("3d4e44Eb1374240CE5F1B871ab261CD16335B76a"),
    v3_migrator_address: Some(address!("23cF10b1ee3AdfCA73B0eF17C07F7577e7ACd2d7")),
    nonfungible_position_manager_address: Some(address!(
        "03a520b32C04BF3bEEf7BEb72E919cf822Ed34f1"
    )),
    tick_lens_address: Some(address!("0CdeE061c75D43c82520eD998C23ac2991c9ac6d")),
    swap_router02_address: Some(address!("2626664c2603336E57B271c5C0b26F421741e481")),
    v1_mixed_route_quoter_address: None,
};

/// Base Goerli v3 addresses
pub const BASE_GOERLI_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("9323c1d6D800ed51Bd7C6B216cfBec678B7d0BC2"),
    multicall_address: address!("B206027a9E0E13F05eBEFa5D2402Bab3eA716439"),
    quoter_address: address!("edf539058e28E5937dAef3f69cEd0b25fbE66Ae9"),
    v3_migrator_address: Some(address!("3efe5d02a04b7351D671Db7008ec6eBA9AD9e3aE")),
    nonfungible_position_manager_address: Some(address!(
        "3c61369ef0D1D2AFa70d8feC2F31C5D6Ce134F30"
    )),
    tick_lens_address: Some(address!("1acB873Ee909D0c98adB18e4474943249F931b92")),
    swap_router02_address: Some(address!("8357227D4eDc78991Db6FDB9bD6ADE250536dE1d")),
    v1_mixed_route_quoter_address: None,
};

/// Zora addresses
pub const ZORA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("7145F8aeef1f6510E92164038E1B6F8cB2c42Cbb"),
    multicall_address: address!("A51c76bEE6746cB487a7e9312E43e2b8f4A37C15"),
    quoter_address: address!("11867e1b3348F3ce4FcC170BC5af3d23E07E64Df"),
    v3_migrator_address: Some(address!("048352d8dCF13686982C799da63fA6426a9D0b60")),
    nonfungible_position_manager_address: Some(address!(
        "bC91e8DfA3fF18De43853372A3d7dfe585137D78"
    )),
    tick_lens_address: Some(address!("209AAda09D74Ad3B8D0E92910Eaf85D2357e3044")),
    swap_router02_address: Some(address!("7De04c96BE5159c3b5CeffC82aa176dc81281557")),
    v1_mixed_route_quoter_address: None,
};

/// Zora Sepolia addresses
pub const ZORA_SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("4324A677D74764f46f33ED447964252441aA8Db6"),
    multicall_address: address!("A1E7e3A69671C4494EC59Dbd442de930a93F911A"),
    quoter_address: address!("C195976fEF0985886E37036E2DF62bF371E12Df0"),
    v3_migrator_address: Some(address!("65ef259b31bf1d977c37e9434658694267674897")),
    nonfungible_position_manager_address: Some(address!(
        "B8458EaAe43292e3c1F7994EFd016bd653d23c20"
    )),
    tick_lens_address: Some(address!("23C0F71877a1Fc4e20A78018f9831365c85f3064")),
    swap_router02_address: None,
    v1_mixed_route_quoter_address: None,
};

/// Rootstock addresses
pub const ROOTSTOCK_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory_address: address!("aF37EC98A00FD63689CF3060BF3B6784E00caD82"),
    multicall_address: address!("996a9858cDfa45Ad68E47c9A30a7201E29c6a386"),
    quoter_address: address!("b51727c996C68E60F598A923a5006853cd2fEB31"),
    v3_migrator_address: Some(address!("16678977CA4ec3DAD5efc7b15780295FE5f56162")),
    nonfungible_position_manager_address: Some(address!(
        "9d9386c042F194B460Ec424a1e57ACDE25f5C4b1"
    )),
    tick_lens_address: Some(address!("55B9dF5bF68ADe972191a91980459f48ecA16afC")),
    swap_router02_address: Some(address!("0B14ff67f0014046b4b99057Aec4509640b3947A")),
    v1_mixed_route_quoter_address: None,
};

lazy_static! {
    pub static ref CHAIN_TO_ADDRESSES_MAP: ChainMap = {
        let mut new_map = ChainMap::new();
        new_map.insert(ChainId::BNB as u64, BNB_ADDRESSES);

        new_map.insert(ChainId::AVALANCHE as u64, AVALANCHE_ADDRESSES);
        new_map.insert(ChainId::MAINNET as u64, *MAINNET_ADDRESSES);
        new_map.insert(ChainId::SEPOLIA as u64, SEPOLIA_ADDRESSES);
        new_map.insert(ChainId::GOERLI as u64, *GOERLI_ADDRESSES);
        new_map.insert(ChainId::ARBITRUMONE as u64, *ARBITUM_ONE_ADDRESSES);
        new_map.insert(ChainId::ARBITRUMGOERLI as u64, ARBITRUM_GOERLI_ADDRESSES);
        new_map.insert(ChainId::ARBITRUMSEPOLIA as u64, ARBITRUM_SEPOLIA_ADDRESSES);
        new_map.insert(ChainId::CELO as u64, CELO_ADDRESSES);
        new_map.insert(ChainId::CELOALFAJORES as u64, CELO_ADDRESSES);

        new_map.insert(ChainId::POLYGON as u64, *POLYGON_ADDRESSES);
        new_map.insert(ChainId::POLYGONMUMBAI as u64, *POLYGON_ADDRESSES);
        new_map.insert(ChainId::OPTIMISM as u64, *OPTIMISM_ADDRESSES);
        new_map.insert(ChainId::OPTIMISMGOERLI as u64, OPTIMISM_GOERLI_ADDRESSES);
        new_map.insert(ChainId::OPTIMISMSEPOLIA as u64, OPTIMISM_SEPOLIA_ADDRESSES);
        new_map.insert(ChainId::BASEGOERLI as u64, BASE_GOERLI_ADDRESSES);
        new_map.insert(ChainId::BASE as u64, BASE_ADDRESSES);
        new_map.insert(ChainId::ZORA as u64, ZORA_ADDRESSES);
        new_map.insert(ChainId::ZORASEPOLIA as u64, ZORA_SEPOLIA_ADDRESSES);
        new_map.insert(ChainId::ROOTSTOCK as u64, ROOTSTOCK_ADDRESSES);
        new_map
    };
}

lazy_static! {
    /// V3 Contract Addresses
    pub static ref V3_CORE_FACTORY_ADDRESSES: ChainAddress = {
        let mut chain_add = ChainAddress::new();
        for chain_id in SUPPORTED_CHAINS {
            chain_add.insert(
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP
                    .get(&(chain_id as u64))
                    .unwrap()
                    .v3_core_factory_address,
            );
        }
        chain_add
    };
}

lazy_static! {
    /// V3 Contract Addresses
    pub static ref V3_MIGRATOR_ADDRESSES: ChainAddress = {
        let mut chain_add = ChainAddress::new();
        for chain_id in SUPPORTED_CHAINS {
            chain_add.insert(
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP
                    .get(&(chain_id as u64))
                    .unwrap()
                    .v3_migrator_address
                    .unwrap(),
            );
        }
        chain_add
    };
}

lazy_static! {
    /// V3 Contract Addresses
    pub static ref MULTICALL_ADDRESSES: ChainAddress = {
        let mut chain_add = ChainAddress::new();
        for chain_id in SUPPORTED_CHAINS {
            chain_add.insert(
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP
                    .get(&(chain_id as u64))
                    .unwrap()
                    .multicall_address,
            );
        }
        chain_add
    };
}

lazy_static! {
/// The oldest V0 governance address
pub static ref GOVERNANCE_ALPHA_V0_ADDRESSES: AddressMap = {
    construct_same_address_map(address!("5e4be8Bc9637f0EAA1A755019e06A68ce081D58F"), &[])
};
}

lazy_static! {
/// The older V1 governance address
pub static ref GOVERNANCE_ALPHA_V1_ADDRESSES: AddressMap = {
    let mut new_map = AddressMap::new();
    new_map.insert(
        ChainId::MAINNET as u64,
        address!("C4e172459f1E7939D522503B81AFAaC1014CE6F6"),
    );
    new_map
};
}

lazy_static! {
/// The latest governor bravo that is currently admin of timelock
pub static ref GOVERNANCE_BRAVO_ADDRESSES: AddressMap = {
    let mut new_map = AddressMap::new();
    new_map.insert(
        ChainId::MAINNET as u64,
        address!("408ED6354d4973f66138C91495F2f2FCbd8724C3"),
    );
    new_map
};
}

lazy_static! {
    pub static ref TIMELOCK_ADDRESSES: AddressMap =
        construct_same_address_map(address!("1a9C8182C09F50C8318d769245beA52c32BE35BC"), &[]);
}

lazy_static! {
    pub static ref MERKLE_DISTRIBUTOR_ADDRESS: AddressMap = {
        let mut new_map = AddressMap::new();
        new_map.insert(
            ChainId::MAINNET as u64,
            address!("090D4613473dEE047c3f2706764f49E0821D256e"),
        );
        new_map
    };
}

lazy_static! {
    pub static ref ARGENT_WALLET_DETECTOR_ADDRESS: AddressMap = {
        let mut new_map = AddressMap::new();
        new_map.insert(
            ChainId::MAINNET as u64,
            address!("eca4B0bDBf7c55E9b7925919d03CbF8Dc82537E8"),
        );
        new_map
    };
}

lazy_static! {
    pub static ref QUOTER_ADDRESSESES: ChainAddress = {
        let mut chain_add = ChainAddress::new();
        for chain_id in SUPPORTED_CHAINS {
            chain_add.insert(
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP
                    .get(&(chain_id as u64))
                    .unwrap()
                    .quoter_address,
            );
        }
        chain_add
    };
}

lazy_static! {
    pub static ref NONFUNGIBLE_POSITION_MANAGER_ADDRESSES: ChainAddress = {
        let mut chain_add = ChainAddress::new();
        for chain_id in SUPPORTED_CHAINS {
            if CHAIN_TO_ADDRESSES_MAP
                .get(&(chain_id as u64))
                .unwrap()
                .nonfungible_position_manager_address
                .is_some()
            {
                chain_add.insert(
                    chain_id as u64,
                    CHAIN_TO_ADDRESSES_MAP
                        .get(&(chain_id as u64))
                        .unwrap()
                        .nonfungible_position_manager_address
                        .unwrap(),
                );
            }
        }
        chain_add
    };
}

lazy_static! {
    pub static ref ENS_RESGISTER_ADDRESS: AddressMap =
        construct_same_address_map(address!("00000000000C2E074eC69A0dFb2997BA6C7d2e1e"), &[]);
}

lazy_static! {
    pub static ref SOCKS_CONTROLLER_ADDRESSES: AddressMap = {
        let mut new_map = AddressMap::new();
        new_map.insert(
            ChainId::MAINNET as u64,
            address!("65770b5283117639760beA3F867b69b3697a91dd"),
        );
        new_map
    };
}

lazy_static! {
    pub static ref TICK_LENS_ADDRESSES: ChainAddress = {
        let mut chain_add = ChainAddress::new();
        for chain_id in SUPPORTED_CHAINS {
            if CHAIN_TO_ADDRESSES_MAP
                .get(&(chain_id as u64))
                .unwrap()
                .tick_lens_address
                .is_some()
            {
                chain_add.insert(
                    chain_id as u64,
                    CHAIN_TO_ADDRESSES_MAP
                        .get(&(chain_id as u64))
                        .unwrap()
                        .tick_lens_address
                        .unwrap(),
                );
            }
        }
        chain_add
    };
}

lazy_static! {
    pub static ref V1_MIXED_ROUTE_QUOTER_ADDRESSES: ChainAddress = {
        let mut chain_add = ChainAddress::new();
        for chain_id in SUPPORTED_CHAINS {
            if CHAIN_TO_ADDRESSES_MAP
                .get(&(chain_id as u64))
                .unwrap()
                .v1_mixed_route_quoter_address
                .is_some()
            {
                chain_add.insert(
                    chain_id as u64,
                    CHAIN_TO_ADDRESSES_MAP
                        .get(&(chain_id as u64))
                        .unwrap()
                        .v1_mixed_route_quoter_address
                        .unwrap(),
                );
            }
        }
        chain_add
    };
}

pub fn swap_router02_address(chain_id: u64) -> Address {
    if CHAIN_TO_ADDRESSES_MAP.contains_key(&chain_id)
        && CHAIN_TO_ADDRESSES_MAP
            .get(&chain_id)
            .unwrap()
            .swap_router02_address
            .is_some()
    {
        CHAIN_TO_ADDRESSES_MAP
            .get(&chain_id)
            .unwrap()
            .swap_router02_address
            .unwrap()
    } else {
        address!("68b3465833fb72A70ecDF485E0e4C7bD8665Fc45")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_router_02_addresses_base() {
        let address = swap_router02_address(ChainId::BASE as u64);
        assert_eq!(
            address,
            address!("2626664c2603336E57B271c5C0b26F421741e481")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_base_goerli() {
        let address = swap_router02_address(ChainId::BASEGOERLI as u64);
        assert_eq!(
            address,
            address!("8357227D4eDc78991Db6FDB9bD6ADE250536dE1d")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_avalanche() {
        let address = swap_router02_address(ChainId::AVALANCHE as u64);
        assert_eq!(
            address,
            address!("bb00FF08d01D300023C629E8fFfFcb65A5a578cE")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_bnb() {
        let address = swap_router02_address(ChainId::BNB as u64);
        assert_eq!(
            address,
            address!("B971eF87ede563556b2ED4b1C0b0019111Dd85d2")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_arbritum_goerli() {
        let address = swap_router02_address(ChainId::ARBITRUMGOERLI as u64);
        assert_eq!(
            address,
            address!("68b3465833fb72A70ecDF485E0e4C7bD8665Fc45")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_optimism_sepolia() {
        let address = swap_router02_address(ChainId::OPTIMISMSEPOLIA as u64);
        assert_eq!(
            address,
            address!("94cC0AaC535CCDB3C01d6787D6413C739ae12bc4")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_sepolia() {
        let address = swap_router02_address(ChainId::SEPOLIA as u64);
        assert_eq!(
            address,
            address!("3bFA4769FB09eefC5a80d6E87c3B9C650f7Ae48E")
        );
    }
}
