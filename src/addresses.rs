use crate::prelude::*;
use alloy_primitives::address;
use lazy_static::lazy_static;

pub type AddressMap = HashMap<u64, Address>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChainAddresses {
    v3_core_factory: Address,
    multicall: Address,
    quoter: Address,
    quoter_v2: Address,
    v3_migrator: Option<Address>,
    nonfungible_position_manager: Address,
    tick_lens: Option<Address>,
    swap_router02: Option<Address>,
    mixed_route_quoter_v1: Option<Address>,
    mixed_route_quoter_v2: Option<Address>,

    v4_pool_manager: Option<Address>,
    v4_position_manager: Option<Address>,
    v4_state_view: Option<Address>,
    v4_quoter: Option<Address>,
}

pub const DEFAULT_NETWORKS: [ChainId; 3] = [ChainId::MAINNET, ChainId::GOERLI, ChainId::SEPOLIA];

#[inline]
fn construct_same_address_map(address: Address, additional_networks: &[ChainId]) -> AddressMap {
    let mut networks = DEFAULT_NETWORKS.to_vec();
    networks.extend_from_slice(additional_networks);
    AddressMap::from_iter(
        networks
            .into_iter()
            .map(|chain_id| (chain_id as u64, address)),
    )
}

lazy_static! {
    pub static ref UNI_ADDRESSES: AddressMap = construct_same_address_map(
        address!("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984"),
        &[
            ChainId::OPTIMISM,
            ChainId::ARBITRUM_ONE,
            ChainId::POLYGON,
            ChainId::POLYGON_MUMBAI,
            ChainId::SEPOLIA,
        ]
    );
}

pub const UNISWAP_NFT_AIRDROP_CLAIM_ADDRESS: Address =
    address!("0x8B799381ac40b838BBA4131ffB26197C432AFe78");

pub const V2_FACTORY_ADDRESS: Address = address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");

lazy_static! {
    pub static ref V2_FACTORY_ADDRESSES: AddressMap = {
        AddressMap::from_iter([
            (ChainId::MAINNET as u64, V2_FACTORY_ADDRESS),
            (ChainId::GOERLI as u64, V2_FACTORY_ADDRESS),
            (
                ChainId::SEPOLIA as u64,
                address!("0xF62c03E08ada871A0bEb309762E260a7a6a880E6"),
            ),
            (
                ChainId::OPTIMISM as u64,
                address!("0x0c3c1c532F1e39EdF36BE9Fe0bE1410313E074Bf"),
            ),
            (
                ChainId::ARBITRUM_ONE as u64,
                address!("0xf1D7CC64Fb4452F05c498126312eBE29f30Fbcf9"),
            ),
            (
                ChainId::AVALANCHE as u64,
                address!("0x9e5A52f57b3038F1B8EeE45F28b3C1967e22799C"),
            ),
            (
                ChainId::BASE_SEPOLIA as u64,
                address!("0x7Ae58f10f7849cA6F5fB71b7f45CB416c9204b1e"),
            ),
            (
                ChainId::BASE as u64,
                address!("0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6"),
            ),
            (
                ChainId::BNB as u64,
                address!("0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6"),
            ),
            (
                ChainId::POLYGON as u64,
                address!("0x9e5A52f57b3038F1B8EeE45F28b3C1967e22799C"),
            ),
            (
                ChainId::CELO as u64,
                address!("0x79a530c8e2fA8748B7B40dd3629C0520c2cCf03f"),
            ),
            (
                ChainId::BLAST as u64,
                address!("0x5C346464d33F90bABaf70dB6388507CC889C1070"),
            ),
            (ChainId::WORLDCHAIN as u64, V2_FACTORY_ADDRESS),
            (ChainId::UNICHAIN_SEPOLIA as u64, V2_FACTORY_ADDRESS),
            (
                ChainId::UNICHAIN as u64,
                address!("0x1f98400000000000000000000000000000000002"),
            ),
            (
                ChainId::MONAD_TESTNET as u64,
                address!("0x733e88f248b742db6c14c0b1713af5ad7fdd59d0"),
            ),
        ])
    };
}

pub const V2_ROUTER_ADDRESS: Address = address!("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");

lazy_static! {
    pub static ref V2_ROUTER_ADDRESSES: AddressMap = {
        AddressMap::from_iter([
            (ChainId::MAINNET as u64, V2_ROUTER_ADDRESS),
            (ChainId::GOERLI as u64, V2_ROUTER_ADDRESS),
            (
                ChainId::ARBITRUM_ONE as u64,
                address!("0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
            ),
            (
                ChainId::OPTIMISM as u64,
                address!("0x4a7b5da61326a6379179b40d00f57e5bbdc962c2"),
            ),
            (
                ChainId::BASE_SEPOLIA as u64,
                address!("0x1689E7B1F10000AE47eBfE339a4f69dECd19F602"),
            ),
            (
                ChainId::BASE as u64,
                address!("0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
            ),
            (
                ChainId::AVALANCHE as u64,
                address!("0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
            ),
            (
                ChainId::BNB as u64,
                address!("0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
            ),
            (
                ChainId::POLYGON as u64,
                address!("0xedf6066a2b290c185783862c7f4776a2c8077ad1"),
            ),
            (
                ChainId::BLAST as u64,
                address!("0xBB66Eb1c5e875933D44DAe661dbD80e5D9B03035"),
            ),
            (
                ChainId::WORLDCHAIN as u64,
                address!("0xf164fC0Ec4E93095b804a4795bBe1e041497b92a"),
            ),
            (
                ChainId::UNICHAIN_SEPOLIA as u64,
                address!("0x920b806E40A00E02E7D2b94fFc89860fDaEd3640"),
            ),
            (
                ChainId::UNICHAIN as u64,
                address!("0x284f11109359a7e1306c3e447ef14d38400063ff"),
            ),
            (
                ChainId::MONAD_TESTNET as u64,
                address!("0xfb8e1c3b833f9e67a71c859a132cf783b645e436"),
            ),
        ])
    };
}

impl ChainAddresses {
    /// Networks that share most of the same addresses i.e. Mainnet, Goerli, Optimism, Arbitrum,
    /// Polygon
    const fn default() -> Self {
        Self {
            v3_core_factory: address!("0x1F98431c8aD98523631AE4a59f267346ea31F984"),
            multicall: address!("0x1F98415757620B543A52E61c46B32eB19261F984"),
            quoter: address!("0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6"),
            quoter_v2: address!("0x61fFE014bA17989E743c5F6cB21bF9697530B21e"),
            v3_migrator: Some(address!("0xA5644E29708357803b5A882D272c41cC0dF92B34")),
            nonfungible_position_manager: address!("0xC36442b4a4522E871399CD717aBDD847Ab11FE88"),
            tick_lens: Some(address!("0xbfd8137f7d1516D3ea5cA83523914859ec47F573")),
            swap_router02: None,
            mixed_route_quoter_v1: None,
            mixed_route_quoter_v2: None,
            v4_pool_manager: None,
            v4_position_manager: None,
            v4_state_view: None,
            v4_quoter: None,
        }
    }
}

const MAINNET_ADDRESSES: ChainAddresses = ChainAddresses {
    mixed_route_quoter_v1: Some(address!("0x84E44095eeBfEC7793Cd7d5b57B7e401D7f1cA2E")),

    v4_pool_manager: Some(address!("0x000000000004444c5dc75cB358380D2e3dE08A90")),
    v4_position_manager: Some(address!("0xbd216513d74c8cf14cf4747e6aaa6420ff64ee9e")),
    v4_state_view: Some(address!("0x7ffe42c4a5deea5b0fec41c94c136cf115597227")),
    v4_quoter: Some(address!("0x52f0e24d1c21c8a0cb1e5a5dd6198556bd9e1203")),
    ..ChainAddresses::default()
};

const GOERLI_ADDRESSES: ChainAddresses = ChainAddresses {
    mixed_route_quoter_v1: Some(address!("0xBa60b6e6fF25488308789E6e0A65D838be34194e")),
    ..ChainAddresses::default()
};

const OPTIMISM_ADDRESSES: ChainAddresses = ChainAddresses {
    v4_pool_manager: Some(address!("0x9a13f98cb987694c9f086b1f5eb990eea8264ec3")),
    v4_position_manager: Some(address!("0x3c3ea4b57a46241e54610e5f022e5c45859a1017")),
    v4_state_view: Some(address!("0xc18a3169788f4f75a170290584eca6395c75ecdb")),
    v4_quoter: Some(address!("0x1f3131a13296fb91c90870043742c3cdbff1a8d7")),
    ..ChainAddresses::default()
};

const ARBITUM_ONE_ADDRESSES: ChainAddresses = ChainAddresses {
    multicall: address!("0xadF885960B47eA2CD9B55E6DAc6B42b7Cb2806dB"),

    v4_pool_manager: Some(address!("0x360e68faccca8ca495c1b759fd9eee466db9fb32")),
    v4_position_manager: Some(address!("0xd88f38f930b7952f2db2432cb002e7abbf3dd869")),
    v4_state_view: Some(address!("0x76fd297e2d437cd7f76d50f01afe6160f86e9990")),
    v4_quoter: Some(address!("0x3972c00f7ed4885e145823eb7c655375d275a1c5")),
    ..ChainAddresses::default()
};

const POLYGON_ADDRESSES: ChainAddresses = ChainAddresses {
    v4_pool_manager: Some(address!("0x67366782805870060151383f4bbff9dab53e5cd6")),
    v4_position_manager: Some(address!("0x1ec2ebf4f37e7363fdfe3551602425af0b3ceef9")),
    v4_state_view: Some(address!("0x5ea1bd7974c8a611cbab0bdcafcb1d9cc9b3ba5a")),
    v4_quoter: Some(address!("0xb3d5c3dfc3a7aebff71895a7191796bffc2c81b9")),
    ..ChainAddresses::default()
};

const CELO_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0xAfE208a311B21f13EF87E33A90049fC17A7acDEc"),
    multicall: address!("0x633987602DE5C4F337e3DbF265303A1080324204"),
    quoter: address!("0x82825d0554fA07f7FC52Ab63c961F330fdEFa8E8"),
    quoter_v2: address!("0x82825d0554fA07f7FC52Ab63c961F330fdEFa8E8"),
    v3_migrator: Some(address!("0x3cFd4d48EDfDCC53D3f173F596f621064614C582")),
    nonfungible_position_manager: address!("0x3d79EdAaBC0EaB6F08ED885C05Fc0B014290D95A"),
    tick_lens: Some(address!("0x5f115D9113F88e0a0Db1b5033D90D4a9690AcD3D")),
    swap_router02: Some(address!("0x5615CDAb10dc425a742d643d949a7F474C01abc4")),
    ..ChainAddresses::default()
};

const BNB_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0xdB1d10011AD0Ff90774D0C6Bb92e5C5c8b4461F7"),
    multicall: address!("0x963Df249eD09c358A4819E39d9Cd5736c3087184"),
    quoter: address!("0x78D78E420Da98ad378D7799bE8f4AF69033EB077"),
    quoter_v2: address!("0x78D78E420Da98ad378D7799bE8f4AF69033EB077"),
    v3_migrator: Some(address!("0x32681814957e0C13117ddc0c2aba232b5c9e760f")),
    nonfungible_position_manager: address!("0x7b8A01B39D58278b5DE7e48c8449c9f4F5170613"),
    tick_lens: Some(address!("0xD9270014D396281579760619CCf4c3af0501A47C")),
    swap_router02: Some(address!("0xB971eF87ede563556b2ED4b1C0b0019111Dd85d2")),

    v4_pool_manager: Some(address!("0x28e2ea090877bf75740558f6bfb36a5ffee9e9df")),
    v4_position_manager: Some(address!("0x7a4a5c919ae2541aed11041a1aeee68f1287f95b")),
    v4_state_view: Some(address!("0xd13dd3d6e93f276fafc9db9e6bb47c1180aee0c4")),
    v4_quoter: Some(address!("0x9f75dd27d6664c475b90e105573e550ff69437b0")),
    ..ChainAddresses::default()
};

const OPTIMISM_GOERLI_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0xB656dA17129e7EB733A557f4EBc57B76CFbB5d10"),
    multicall: address!("0x07F2D8a2a02251B62af965f22fC4744A5f96BCCd"),
    quoter: address!("0x9569CbA925c8ca2248772A9A4976A516743A246F"),
    v3_migrator: Some(address!("0xf6c55fBe84B1C8c3283533c53F51bC32F5C7Aba8")),
    nonfungible_position_manager: address!("0x39Ca85Af2F383190cBf7d7c41ED9202D27426EF6"),
    tick_lens: Some(address!("0xe6140Bd164b63E8BfCfc40D5dF952f83e171758e")),
    ..ChainAddresses::default()
};

const OPTIMISM_SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x8CE191193D15ea94e11d327b4c7ad8bbE520f6aF"),
    multicall: address!("0x80e4e06841bb76AA9735E0448cB8d003C0EF009a"),
    quoter: address!("0x0FBEa6cf957d95ee9313490050F6A0DA68039404"),
    quoter_v2: address!("0x0FBEa6cf957d95ee9313490050F6A0DA68039404"),
    v3_migrator: Some(address!("0xE7EcbAAaA54D007A00dbb6c1d2f150066D69dA07")),
    nonfungible_position_manager: address!("0xdA75cEf1C93078e8b736FCA5D5a30adb97C8957d"),
    tick_lens: Some(address!("0xCb7f54747F58F8944973cea5b8f4ac2209BadDC5")),
    swap_router02: Some(address!("0x94cC0AaC535CCDB3C01d6787D6413C739ae12bc4")),
    ..ChainAddresses::default()
};

const ARBITRUM_GOERLI_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x4893376342d5D7b3e31d4184c08b265e5aB2A3f6"),
    multicall: address!("0x8260CB40247290317a4c062F3542622367F206Ee"),
    quoter: address!("0x1dd92b83591781D0C6d98d07391eea4b9a6008FA"),
    v3_migrator: Some(address!("0xA815919D2584Ac3F76ea9CB62E6Fd40a43BCe0C3")),
    nonfungible_position_manager: address!("0x622e4726a167799826d1E1D150b076A7725f5D81"),
    tick_lens: Some(address!("0xb52429333da969a0C79a60930a4Bf0020E5D1DE8")),
    ..ChainAddresses::default()
};

const ARBITRUM_SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x248AB79Bbb9bC29bB72f7Cd42F17e054Fc40188e"),
    multicall: address!("0x2B718b475e385eD29F56775a66aAB1F5cC6B2A0A"),
    quoter: address!("0x2779a0CC1c3e0E44D2542EC3e79e3864Ae93Ef0B"),
    quoter_v2: address!("0x2779a0CC1c3e0E44D2542EC3e79e3864Ae93Ef0B"),
    v3_migrator: Some(address!("0x398f43ef2c67B941147157DA1c5a868E906E043D")),
    nonfungible_position_manager: address!("0x6b2937Bde17889EDCf8fbD8dE31C3C2a70Bc4d65"),
    tick_lens: Some(address!("0x0fd18587734e5C2dcE2dccDcC7DD1EC89ba557d9")),
    swap_router02: Some(address!("0x101F443B4d1b059569D643917553c771E1b9663E")),

    v4_pool_manager: Some(address!("0xFB3e0C6F74eB1a21CC1Da29aeC80D2Dfe6C9a317")),
    v4_position_manager: Some(address!("0xAc631556d3d4019C95769033B5E719dD77124BAc")),
    v4_state_view: Some(address!("0x9d467fa9062b6e9b1a46e26007ad82db116c67cb")),
    v4_quoter: Some(address!("0x7de51022d70a725b508085468052e25e22b5c4c9")),
    ..ChainAddresses::default()
};

const SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x0227628f3F023bb0B980b67D528571c95c6DaC1c"),
    multicall: address!("0xD7F33bCdb21b359c8ee6F0251d30E94832baAd07"),
    quoter: address!("0xEd1f6473345F45b75F8179591dd5bA1888cf2FB3"),
    quoter_v2: address!("0xEd1f6473345F45b75F8179591dd5bA1888cf2FB3"),
    v3_migrator: Some(address!("0x729004182cF005CEC8Bd85df140094b6aCbe8b15")),
    nonfungible_position_manager: address!("0x1238536071E1c677A632429e3655c799b22cDA52"),
    tick_lens: Some(address!("0x0b343475d44EC2b4b8243EBF81dc888BF0A14b36")),
    swap_router02: Some(address!("0x3bFA4769FB09eefC5a80d6E87c3B9C650f7Ae48E")),
    mixed_route_quoter_v2: Some(address!("0x4745f77b56a0e2294426e3936dc4fab68d9543cd")),

    v4_pool_manager: Some(address!("0xE03A1074c86CFeDd5C142C4F04F1a1536e203543")),
    v4_position_manager: Some(address!("0x429ba70129df741B2Ca2a85BC3A2a3328e5c09b4")),
    v4_state_view: Some(address!("0xE1Dd9c3fA50EDB962E442f60DfBc432e24537E4C")),
    v4_quoter: Some(address!("0x61B3f2011A92d183C7dbaDBdA940a7555Ccf9227")),
    ..ChainAddresses::default()
};

const AVALANCHE_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x740b1c1de25031C31FF4fC9A62f554A55cdC1baD"),
    multicall: address!("0x0139141Cd4Ee88dF3Cdb65881D411bAE271Ef0C2"),
    quoter: address!("0xbe0F5544EC67e9B3b2D979aaA43f18Fd87E6257F"),
    quoter_v2: address!("0xbe0F5544EC67e9B3b2D979aaA43f18Fd87E6257F"),
    v3_migrator: Some(address!("0x44f5f1f5E452ea8d29C890E8F6e893fC0f1f0f97")),
    nonfungible_position_manager: address!("0x655C406EBFa14EE2006250925e54ec43AD184f8B"),
    tick_lens: Some(address!("0xEB9fFC8bf81b4fFd11fb6A63a6B0f098c6e21950")),
    swap_router02: Some(address!("0xbb00FF08d01D300023C629E8fFfFcb65A5a578cE")),

    v4_pool_manager: Some(address!("0x06380c0e0912312b5150364b9dc4542ba0dbbc85")),
    v4_position_manager: Some(address!("0xb74b1f14d2754acfcbbe1a221023a5cf50ab8acd")),
    v4_state_view: Some(address!("0xc3c9e198c735a4b97e3e683f391ccbdd60b69286")),
    v4_quoter: Some(address!("0xbe40675bb704506a3c2ccfb762dcfd1e979845c2")),
    ..ChainAddresses::default()
};

const BASE_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x33128a8fC17869897dcE68Ed026d694621f6FDfD"),
    multicall: address!("0x091e99cb1C49331a94dD62755D168E941AbD0693"),
    quoter: address!("0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a"),
    quoter_v2: address!("0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a"),
    v3_migrator: Some(address!("0x23cF10b1ee3AdfCA73B0eF17C07F7577e7ACd2d7")),
    nonfungible_position_manager: address!("0x03a520b32C04BF3bEEf7BEb72E919cf822Ed34f1"),
    tick_lens: Some(address!("0x0CdeE061c75D43c82520eD998C23ac2991c9ac6d")),
    swap_router02: Some(address!("0x2626664c2603336E57B271c5C0b26F421741e481")),
    mixed_route_quoter_v1: Some(address!("0xe544efae946f0008ae9a8d64493efa7886b73776")),

    v4_pool_manager: Some(address!("0x498581ff718922c3f8e6a244956af099b2652b2b")),
    v4_position_manager: Some(address!("0x7c5f5a4bbd8fd63184577525326123b519429bdc")),
    v4_state_view: Some(address!("0xa3c0c9b65bad0b08107aa264b0f3db444b867a71")),
    v4_quoter: Some(address!("0x0d5e0f971ed27fbff6c2837bf31316121532048d")),
    ..ChainAddresses::default()
};

const BASE_GOERLI_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x9323c1d6D800ed51Bd7C6B216cfBec678B7d0BC2"),
    multicall: address!("0xB206027a9E0E13F05eBEFa5D2402Bab3eA716439"),
    quoter: address!("0xedf539058e28E5937dAef3f69cEd0b25fbE66Ae9"),
    v3_migrator: Some(address!("0x3efe5d02a04b7351D671Db7008ec6eBA9AD9e3aE")),
    nonfungible_position_manager: address!("0x3c61369ef0D1D2AFa70d8feC2F31C5D6Ce134F30"),
    tick_lens: Some(address!("0x1acB873Ee909D0c98adB18e4474943249F931b92")),
    swap_router02: Some(address!("0x8357227D4eDc78991Db6FDB9bD6ADE250536dE1d")),
    ..ChainAddresses::default()
};

const BASE_SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24"),
    multicall: address!("0xd867e273eAbD6c853fCd0Ca0bFB6a3aE6491d2C1"),
    quoter: address!("0xC5290058841028F1614F3A6F0F5816cAd0df5E27"),
    quoter_v2: address!("0xC5290058841028F1614F3A6F0F5816cAd0df5E27"),
    v3_migrator: Some(address!("0xCbf8b7f80800bd4888Fbc7bf1713B80FE4E23E10")),
    nonfungible_position_manager: address!("0x27F971cb582BF9E50F397e4d29a5C7A34f11faA2"),
    tick_lens: Some(address!("0xedf6066a2b290C185783862C7F4776A2C8077AD1")),
    swap_router02: Some(address!("0x94cC0AaC535CCDB3C01d6787D6413C739ae12bc4")),

    v4_pool_manager: Some(address!("0x05E73354cFDd6745C338b50BcFDfA3Aa6fA03408")),
    v4_position_manager: Some(address!("0x4b2c77d209d3405f41a037ec6c77f7f5b8e2ca80")),
    v4_state_view: Some(address!("0x571291b572ed32ce6751a2cb2486ebee8defb9b4")),
    v4_quoter: Some(address!("0x4a6513c898fe1b2d0e78d3b0e0a4a151589b1cba")),
    ..ChainAddresses::default()
};

const ZORA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x7145F8aeef1f6510E92164038E1B6F8cB2c42Cbb"),
    multicall: address!("0xA51c76bEE6746cB487a7e9312E43e2b8f4A37C15"),
    quoter: address!("0x11867e1b3348F3ce4FcC170BC5af3d23E07E64Df"),
    quoter_v2: address!("0x11867e1b3348F3ce4FcC170BC5af3d23E07E64Df"),
    v3_migrator: Some(address!("0x048352d8dCF13686982C799da63fA6426a9D0b60")),
    nonfungible_position_manager: address!("0xbC91e8DfA3fF18De43853372A3d7dfe585137D78"),
    tick_lens: Some(address!("0x209AAda09D74Ad3B8D0E92910Eaf85D2357e3044")),
    swap_router02: Some(address!("0x7De04c96BE5159c3b5CeffC82aa176dc81281557")),

    v4_pool_manager: Some(address!("0x0575338e4c17006ae181b47900a84404247ca30f")),
    v4_position_manager: Some(address!("0xf66c7b99e2040f0d9b326b3b7c152e9663543d63")),
    v4_state_view: Some(address!("0x385785af07d63b50d0a0ea57c4ff89d06adf7328")),
    v4_quoter: Some(address!("0x5edaccc0660e0a2c44b06e07ce8b915e625dc2c6")),
    ..ChainAddresses::default()
};

const ZORA_SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x4324A677D74764f46f33ED447964252441aA8Db6"),
    multicall: address!("0xA1E7e3A69671C4494EC59Dbd442de930a93F911A"),
    quoter: address!("0xC195976fEF0985886E37036E2DF62bF371E12Df0"),
    v3_migrator: Some(address!("0x65ef259b31bf1d977c37e9434658694267674897")),
    nonfungible_position_manager: address!("0xB8458EaAe43292e3c1F7994EFd016bd653d23c20"),
    tick_lens: Some(address!("0x23C0F71877a1Fc4e20A78018f9831365c85f3064")),
    ..ChainAddresses::default()
};

const ROOTSTOCK_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0xaF37EC98A00FD63689CF3060BF3B6784E00caD82"),
    multicall: address!("0x996a9858cDfa45Ad68E47c9A30a7201E29c6a386"),
    quoter: address!("0xb51727c996C68E60F598A923a5006853cd2fEB31"),
    v3_migrator: Some(address!("0x16678977CA4ec3DAD5efc7b15780295FE5f56162")),
    nonfungible_position_manager: address!("0x9d9386c042F194B460Ec424a1e57ACDE25f5C4b1"),
    tick_lens: Some(address!("0x55B9dF5bF68ADe972191a91980459f48ecA16afC")),
    swap_router02: Some(address!("0x0B14ff67f0014046b4b99057Aec4509640b3947A")),
    ..ChainAddresses::default()
};

const BLAST_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x792edAdE80af5fC680d96a2eD80A44247D2Cf6Fd"),
    multicall: address!("0xdC7f370de7631cE9e2c2e1DCDA6B3B5744Cf4705"),
    quoter: address!("0x6Cdcd65e03c1CEc3730AeeCd45bc140D57A25C77"),
    quoter_v2: address!("0x6Cdcd65e03c1CEc3730AeeCd45bc140D57A25C77"),
    v3_migrator: Some(address!("0x15CA7043CD84C5D21Ae76Ba0A1A967d42c40ecE0")),
    nonfungible_position_manager: address!("0xB218e4f7cF0533d4696fDfC419A0023D33345F28"),
    tick_lens: Some(address!("0x2E95185bCdD928a3e984B7e2D6560Ab1b17d7274")),
    swap_router02: Some(address!("0x549FEB8c9bd4c12Ad2AB27022dA12492aC452B66")),

    v4_pool_manager: Some(address!("0x1631559198a9e474033433b2958dabc135ab6446")),
    v4_position_manager: Some(address!("0x4ad2f4cca2682cbb5b950d660dd458a1d3f1baad")),
    v4_state_view: Some(address!("0x12a88ae16f46dce4e8b15368008ab3380885df30")),
    v4_quoter: Some(address!("0x6f71cdcb0d119ff72c6eb501abceb576fbf62bcf")),
    ..ChainAddresses::default()
};

const ZKSYNC_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x8FdA5a7a8dCA67BBcDd10F02Fa0649A937215422"),
    multicall: address!("0x0c68a7C72f074d1c45C16d41fa74eEbC6D16a65C"),
    quoter: address!("0x8Cb537fc92E26d8EBBb760E632c95484b6Ea3e28"),
    quoter_v2: address!("0x8Cb537fc92E26d8EBBb760E632c95484b6Ea3e28"),
    v3_migrator: Some(address!("0x611841b24E43C4ACfd290B427a3D6cf1A59dac8E")),
    nonfungible_position_manager: address!("0x0616e5762c1E7Dc3723c50663dF10a162D690a86"),
    tick_lens: Some(address!("0xe10FF11b809f8EE07b056B452c3B2caa7FE24f89")),
    swap_router02: Some(address!("0x99c56385daBCE3E81d8499d0b8d0257aBC07E8A3")),
    ..ChainAddresses::default()
};

const WORLDCHAIN_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x7a5028BDa40e7B173C278C5342087826455ea25a"),
    multicall: address!("0x0a22c04215c97E3F532F4eF30e0aD9458792dAB9"),
    quoter: address!("0x10158D43e6cc414deE1Bd1eB0EfC6a5cBCfF244c"),
    quoter_v2: address!("0x10158D43e6cc414deE1Bd1eB0EfC6a5cBCfF244c"),
    v3_migrator: Some(address!("0x9EBDdCBa71C9027E1eB45135672a30bcFEec9de3")),
    nonfungible_position_manager: address!("0xec12a9F9a09f50550686363766Cc153D03c27b5e"),
    tick_lens: Some(address!("0xE61df0CaC9d85876aCE5E3037005D80943570623")),
    swap_router02: Some(address!("0x091AD9e2e6e5eD44c1c66dB50e49A601F9f36cF6")),

    v4_pool_manager: Some(address!("0xb1860d529182ac3bc1f51fa2abd56662b7d13f33")),
    v4_position_manager: Some(address!("0xc585e0f504613b5fbf874f21af14c65260fb41fa")),
    v4_state_view: Some(address!("0x51d394718bc09297262e368c1a481217fdeb71eb")),
    v4_quoter: Some(address!("0x55d235b3ff2daf7c3ede0defc9521f1d6fe6c5c0")),
    ..ChainAddresses::default()
};

const UNICHAIN_SEPOLIA_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x1F98431c8aD98523631AE4a59f267346ea31F984"),
    multicall: address!("0x9D0F15f2cf58655fDDcD1EE6129C547fDaeD01b1"),
    quoter: address!("0x6Dd37329A1A225a6Fca658265D460423DCafBF89"),
    quoter_v2: address!("0x6Dd37329A1A225a6Fca658265D460423DCafBF89"),
    v3_migrator: Some(address!("0xb5FA244C9d6D04B2FBac84418b3c4910ED1Ae5f2")),
    nonfungible_position_manager: address!("0xB7F724d6dDDFd008eFf5cc2834edDE5F9eF0d075"),
    tick_lens: Some(address!("0x5f739c790a48E97eec0efb81bab5D152c0A0ecA0")),
    swap_router02: Some(address!("0xd1AAE39293221B77B0C71fBD6dCb7Ea29Bb5B166")),

    v4_pool_manager: Some(address!("0x00b036b58a818b1bc34d502d3fe730db729e62ac")),
    v4_position_manager: Some(address!("0xf969aee60879c54baaed9f3ed26147db216fd664")),
    v4_state_view: Some(address!("0xc199f1072a74d4e905aba1a84d9a45e2546b6222")),
    v4_quoter: Some(address!("0x56dcd40a3f2d466f48e7f48bdbe5cc9b92ae4472")),
    ..ChainAddresses::default()
};

const UNICHAIN_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x1f98400000000000000000000000000000000003"),
    multicall: address!("0xb7610f9b733e7d45184be3a1bc966960ccc54f0b"),
    quoter: address!("0x565ac8c7863d9bb16d07e809ff49fe5cd467634c"),
    quoter_v2: address!("0x565ac8c7863d9bb16d07e809ff49fe5cd467634c"),
    v3_migrator: Some(address!("0xb9d0c246f306b1aaf02ae6ba112d5ef25e5b60dc")),
    nonfungible_position_manager: address!("0x943e6e07a7e8e791dafc44083e54041d743c46e9"),
    tick_lens: Some(address!("0xd5d76fa166ab8d8ad4c9f61aaa81457b66cbe443")),
    swap_router02: Some(address!("0x73855d06de49d0fe4a9c42636ba96c62da12ff9c")),

    v4_pool_manager: Some(address!("0x1f98400000000000000000000000000000000004")),
    v4_position_manager: Some(address!("0x4529a01c7a0410167c5740c487a8de60232617bf")),
    v4_state_view: Some(address!("0x86e8631a016f9068c3f085faf484ee3f5fdee8f2")),
    v4_quoter: Some(address!("0x333e3c607b141b18ff6de9f258db6e77fe7491e0")),
    ..ChainAddresses::default()
};

const MONAD_TESTNET_ADDRESSES: ChainAddresses = ChainAddresses {
    v3_core_factory: address!("0x961235a9020b05c44df1026d956d1f4d78014276"),
    multicall: address!("0xa707ceb989cc3728551ed0e6e44b718dd114cf44"),
    quoter: address!("0x1ba215c17565de7b0cb7ecab971bcf540c24a862"),
    v3_migrator: Some(address!("0x0a78348b71f8ae8caff2f8f9d4d74a2f36516661")),
    nonfungible_position_manager: address!("0x3dcc735c74f10fe2b9db2bb55c40fbbbf24490f7"),
    tick_lens: Some(address!("0x337478eb6058455ecb3696184b30dd6a29e3a893")),
    swap_router02: Some(address!("0x4c4eabd5fb1d1a7234a48692551eaecff8194ca7")),
    ..ChainAddresses::default()
};

lazy_static! {
    /// A map of chain IDs to their corresponding Uniswap contract addresses.
    ///
    /// This map is used to look up the addresses of various Uniswap contracts
    /// for a given network. The keys in the map are the network IDs, and the values
    /// are the corresponding contract addresses.
    pub static ref CHAIN_TO_ADDRESSES_MAP: HashMap<u64, ChainAddresses> = {
        HashMap::from_iter([
            (ChainId::MAINNET as u64, MAINNET_ADDRESSES),
            (ChainId::OPTIMISM as u64, OPTIMISM_ADDRESSES),
            (ChainId::ARBITRUM_ONE as u64, ARBITUM_ONE_ADDRESSES),
            (ChainId::POLYGON as u64, POLYGON_ADDRESSES),
            (ChainId::POLYGON_MUMBAI as u64, POLYGON_ADDRESSES),
            (ChainId::GOERLI as u64, GOERLI_ADDRESSES),
            (ChainId::CELO as u64, CELO_ADDRESSES),
            (ChainId::CELO_ALFAJORES as u64, CELO_ADDRESSES),
            (ChainId::BNB as u64, BNB_ADDRESSES),
            (ChainId::OPTIMISM_GOERLI as u64, OPTIMISM_GOERLI_ADDRESSES),
            (ChainId::OPTIMISM_SEPOLIA as u64, OPTIMISM_SEPOLIA_ADDRESSES),
            (ChainId::ARBITRUM_GOERLI as u64, ARBITRUM_GOERLI_ADDRESSES),
            (ChainId::ARBITRUM_SEPOLIA as u64, ARBITRUM_SEPOLIA_ADDRESSES),
            (ChainId::SEPOLIA as u64, SEPOLIA_ADDRESSES),
            (ChainId::AVALANCHE as u64, AVALANCHE_ADDRESSES),
            (ChainId::BASE as u64, BASE_ADDRESSES),
            (ChainId::BASE_GOERLI as u64, BASE_GOERLI_ADDRESSES),
            (ChainId::BASE_SEPOLIA as u64, BASE_SEPOLIA_ADDRESSES),
            (ChainId::ZORA as u64, ZORA_ADDRESSES),
            (ChainId::ZORA_SEPOLIA as u64, ZORA_SEPOLIA_ADDRESSES),
            (ChainId::ROOTSTOCK as u64, ROOTSTOCK_ADDRESSES),
            (ChainId::BLAST as u64, BLAST_ADDRESSES),
            (ChainId::ZKSYNC as u64, ZKSYNC_ADDRESSES),
            (ChainId::WORLDCHAIN as u64, WORLDCHAIN_ADDRESSES),
            (ChainId::UNICHAIN_SEPOLIA as u64, UNICHAIN_SEPOLIA_ADDRESSES),
            (ChainId::UNICHAIN as u64, UNICHAIN_ADDRESSES),
            (ChainId::MONAD_TESTNET as u64, MONAD_TESTNET_ADDRESSES),
        ])
    };
}

lazy_static! {
    pub static ref V3_CORE_FACTORY_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.map(|chain_id| {
            (
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)].v3_core_factory,
            )
        }));
}

lazy_static! {
    pub static ref V3_MIGRATOR_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.into_iter().filter_map(|chain_id| {
            CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)]
                .v3_migrator
                .map(|address| (chain_id as u64, address))
        }));
}

lazy_static! {
    pub static ref MULTICALL_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.map(|chain_id| {
            (
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)].multicall,
            )
        }));
}

lazy_static! {
    /// The oldest V0 governance address
    pub static ref GOVERNANCE_ALPHA_V0_ADDRESSES: AddressMap =
        construct_same_address_map(address!("0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F"), &[]);
}

lazy_static! {
    /// The older V1 governance address
    pub static ref GOVERNANCE_ALPHA_V1_ADDRESSES: AddressMap = AddressMap::from_iter([(
        ChainId::MAINNET as u64,
        address!("0xC4e172459f1E7939D522503B81AFAaC1014CE6F6")
    )]);
}

lazy_static! {
    /// The latest governor bravo that is currently admin of timelock
    pub static ref GOVERNANCE_BRAVO_ADDRESSES: AddressMap = AddressMap::from_iter([(
        ChainId::MAINNET as u64,
        address!("0x408ED6354d4973f66138C91495F2f2FCbd8724C3")
    )]);
}

lazy_static! {
    pub static ref TIMELOCK_ADDRESSES: AddressMap =
        construct_same_address_map(address!("0x1a9C8182C09F50C8318d769245beA52c32BE35BC"), &[]);
}

lazy_static! {
    pub static ref MERKLE_DISTRIBUTOR_ADDRESS: AddressMap = AddressMap::from_iter([(
        ChainId::MAINNET as u64,
        address!("0x090D4613473dEE047c3f2706764f49E0821D256e"),
    )]);
}

lazy_static! {
    pub static ref ARGENT_WALLET_DETECTOR_ADDRESS: AddressMap = AddressMap::from_iter([(
        ChainId::MAINNET as u64,
        address!("0xeca4B0bDBf7c55E9b7925919d03CbF8Dc82537E8"),
    )]);
}

lazy_static! {
    pub static ref QUOTER_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.map(|chain_id| {
            (
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)].quoter,
            )
        }));
}

lazy_static! {
    pub static ref QUOTER_V2_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.map(|chain_id| {
            (
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)].quoter_v2,
            )
        }));
}

lazy_static! {
    pub static ref NONFUNGIBLE_POSITION_MANAGER_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.map(|chain_id| {
            (
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)].nonfungible_position_manager,
            )
        }));
}

lazy_static! {
    pub static ref ENS_REGISTRAR_ADDRESSES: AddressMap =
        construct_same_address_map(address!("0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e"), &[]);
}

lazy_static! {
    pub static ref SOCKS_CONTROLLER_ADDRESSES: AddressMap = AddressMap::from_iter([(
        ChainId::MAINNET as u64,
        address!("0x65770b5283117639760beA3F867b69b3697a91dd")
    )]);
}

lazy_static! {
    pub static ref TICK_LENS_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.into_iter().filter_map(|chain_id| {
            CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)]
                .tick_lens
                .map(|address| (chain_id as u64, address))
        }));
}

lazy_static! {
    pub static ref MIXED_ROUTE_QUOTER_V1_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.into_iter().filter_map(|chain_id| {
            CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)]
                .mixed_route_quoter_v1
                .map(|address| (chain_id as u64, address))
        }));
}

lazy_static! {
    pub static ref SWAP_ROUTER_02_ADDRESSES: AddressMap =
        AddressMap::from_iter(SUPPORTED_CHAINS.map(|chain_id| {
            (
                chain_id as u64,
                CHAIN_TO_ADDRESSES_MAP[&(chain_id as u64)]
                    .swap_router02
                    .unwrap_or(address!("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45")),
            )
        }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_router_02_addresses_base() {
        let address = SWAP_ROUTER_02_ADDRESSES[&(ChainId::BASE as u64)];
        assert_eq!(
            address,
            address!("0x2626664c2603336E57B271c5C0b26F421741e481")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_base_goerli() {
        let address = SWAP_ROUTER_02_ADDRESSES[&(ChainId::BASE_GOERLI as u64)];
        assert_eq!(
            address,
            address!("0x8357227D4eDc78991Db6FDB9bD6ADE250536dE1d")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_avalanche() {
        let address = SWAP_ROUTER_02_ADDRESSES[&(ChainId::AVALANCHE as u64)];
        assert_eq!(
            address,
            address!("0xbb00FF08d01D300023C629E8fFfFcb65A5a578cE")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_bnb() {
        let address = SWAP_ROUTER_02_ADDRESSES[&(ChainId::BNB as u64)];
        assert_eq!(
            address,
            address!("0xB971eF87ede563556b2ED4b1C0b0019111Dd85d2")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_arbritum_goerli() {
        let address = SWAP_ROUTER_02_ADDRESSES[&(ChainId::ARBITRUM_GOERLI as u64)];
        assert_eq!(
            address,
            address!("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_optimism_sepolia() {
        let address = SWAP_ROUTER_02_ADDRESSES[&(ChainId::OPTIMISM_SEPOLIA as u64)];
        assert_eq!(
            address,
            address!("0x94cC0AaC535CCDB3C01d6787D6413C739ae12bc4")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_sepolia() {
        let address = SWAP_ROUTER_02_ADDRESSES[&(ChainId::SEPOLIA as u64)];
        assert_eq!(
            address,
            address!("0x3bFA4769FB09eefC5a80d6E87c3B9C650f7Ae48E")
        );
    }

    #[test]
    fn test_swap_router_02_addresses_blast() {
        let address = SWAP_ROUTER_02_ADDRESSES[&(ChainId::BLAST as u64)];
        assert_eq!(
            address,
            address!("0x549FEB8c9bd4c12Ad2AB27022dA12492aC452B66")
        );
    }
}
