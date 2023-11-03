use cw_orch::{
    daemon::ChainInfo,
    prelude::networks::{osmosis::OSMOSIS_1, ARCHWAY_1},
    prelude::*,
    tokio::runtime::Runtime,
};
use cw_orch_polytone::Polytone;
use scripts::helpers::get_deployment_id;
/// This stays unsued and is used for reference for channel creation
pub const POLYTONE_VERSION: &str = "polytone-1";
fn main() {
    let src_chain = ARCHWAY_1;
    let dst_chain = OSMOSIS_1;
    let src_admin_addr = Some("archway1t07t5ejcwtlclnelvtsdf3rx30kxvczlwcg4ks");
    let dst_admin_addr = Some("osmo1t07t5ejcwtlclnelvtsdf3rx30kxvczlng8p24");

    instantiate_two_chains(src_chain, dst_chain, src_admin_addr, dst_admin_addr).unwrap();
}

/// Instantiates a polytone connection between two chains
/// The channel needs to be created by a relayer.
/// It leverages the deployment id of abstract to have multiple connections on the same chain
fn instantiate_two_chains(
    src_chain: ChainInfo,
    dst_chain: ChainInfo,
    src_admin_addr: Option<&str>,
    dst_admin_addr: Option<&str>,
) -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let rt = Runtime::new()?;

    let deployment_id = get_deployment_id(&src_chain, &dst_chain);

    let src_daemon = DaemonBuilder::default()
        .chain(src_chain)
        .handle(rt.handle())
        .deployment_id(deployment_id.clone())
        .build()?;
    let src_polytone = Polytone::load_from(src_daemon)?;
    src_polytone.instantiate_note(src_admin_addr.map(|s| s.to_string()))?;

    let dst_daemon = DaemonBuilder::default()
        .chain(dst_chain)
        .handle(rt.handle())
        .deployment_id(deployment_id)
        .build()?;

    let dst_polytone = Polytone::load_from(dst_daemon)?;
    dst_polytone.instantiate_voice(dst_admin_addr.map(|s| s.to_string()))?;

    log::info!("Started creation of a polytone connection between two chains.");
    log::info!("Instantiated 2 contracts : ");
    log::info!("Note (src_chain) : {}", src_polytone.note.address()?);
    log::info!("Voice (dst_chain) : {}", dst_polytone.voice.address()?);

    Ok(())
}
