use cw_orch::{
    prelude::{networks::*, *},
    tokio::runtime::Runtime,
};
use cw_orch_polytone::Polytone;
use scripts::helpers::get_deployment_id;
/// This stays unsued and is used for reference for channel creation
pub const POLYTONE_VERSION: &str = "polytone-1";
fn main() {
    let src_chain = JUNO_1;
    let mut dst_chain = NEUTRON_1;

    dst_chain.gas_price = 0.075;

    instantiate_two_chains(src_chain, dst_chain).unwrap();
}

/// Instantiates a polytone connection between two chains
/// The channel needs to be created by a relayer.
/// It leverages the deployment id of abstract to have multiple connections on the same chain
fn instantiate_two_chains(src_chain: ChainInfo, dst_chain: ChainInfo) -> anyhow::Result<()> {
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
    src_polytone.instantiate_note(None)?;

    let dst_daemon = DaemonBuilder::default()
        .chain(dst_chain)
        .handle(rt.handle())
        .deployment_id(deployment_id)
        .build()?;

    let dst_polytone = Polytone::load_from(dst_daemon)?;
    dst_polytone.instantiate_voice(None)?;

    log::info!("Started creation of a polytone connection between two chains.");
    log::info!("Instantiated 2 contracts : ");
    log::info!("Note (src_chain) : {}", src_polytone.note.address()?);
    log::info!("Voice (dst_chain) : {}", dst_polytone.voice.address()?);

    Ok(())
}
