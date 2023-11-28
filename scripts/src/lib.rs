use cw_orch::daemon::{ChainInfo, NetworkInfo};

pub mod helpers;

pub const COSMOS_ADMIN_ADDR: &str = "cosmos1t07t5ejcwtlclnelvtsdf3rx30kxvczlmn53u8";
pub const OSMOSIS_ADMIN_ADDR: &str = "osmo1t07t5ejcwtlclnelvtsdf3rx30kxvczlng8p24";
pub const JUNO_ADMIN_ADDR: &str = "juno1t07t5ejcwtlclnelvtsdf3rx30kxvczldph2mm";
pub const TERRA_ADMIN_ADDR: &str = "terra1372gxrnehp5pm5regl0fze0kym3h807tqzg20n";
pub const NEUTRON_ADMIN_ADDR: &str = "neutron1t07t5ejcwtlclnelvtsdf3rx30kxvczllvanxq";
pub const ARCHWAY_ADMIN_ADDR: &str = "archway1t07t5ejcwtlclnelvtsdf3rx30kxvczlwcg4ks";
pub const STARGAZE_ADMIN_ADDR: &str = "stars1t07t5ejcwtlclnelvtsdf3rx30kxvczl00rvhk";

pub const STARGAZE: NetworkInfo = NetworkInfo {
    id: "stargaze",
    pub_address_prefix: "stars",
    coin_type: 118,
};

pub const STARGAZE_1: ChainInfo = ChainInfo {
    chain_id: "stargaze-1",
    gas_denom: "ustars",
    gas_price: 1.0,
    grpc_urls: &["http://stargaze-grpc.polkachu.com:13790"],
    lcd_url: None,
    fcd_url: None,
    network_info: STARGAZE,
    kind: cw_orch::daemon::ChainKind::Mainnet,
};
