use cw_orch::{prelude::*, prelude::networks::{ARCHWAY_1, JUNO_1}, tokio::runtime::Runtime, daemon::ChainInfo};
use cw_orch_polytone::Polytone;


/// Instantiates a polytone connection between two chains
/// The channel needs to be created by a relayer.
/// It leverages the deployment id of abstract to have multiple connections on the same chain
fn instantiate_two_chains(src_chain: ChainInfo, dst_chain: ChainInfo, src_admin_addr: &str, dst_admin_addr: &str) -> anyhow::Result<()>{

    let deployment_id = format!("{}-->{}", src_chain.chain_id, dst_chain.chain_id);
    let rt = Runtime::new()?;

    let src_daemon = DaemonBuilder::default()
        .chain(src_chain)
        .handle(rt.handle())
        .deployment_id(deployment_id.clone())
        .build()?;
    let src_polytone = Polytone::load_from(src_daemon)?;
    src_polytone.instantiate_note(Some(src_admin_addr.to_string()))?;

    let dst_daemon = DaemonBuilder::default()
        .chain(dst_chain)
        .handle(rt.handle())
        .deployment_id(deployment_id)
        .build()?;

    let dst_polytone = Polytone::load_from(dst_daemon)?;
    dst_polytone.instantiate_voice(Some(dst_admin_addr.to_string()))?;

    log::info!("Started creation of a polytone connection between two chains.");
    log::info!("Instantiated 2 contracts : ");
    log::info!("Note (src_chain) : {}", src_polytone.note.address()?);
    log::info!("Voice (dst_chain) : {}", dst_polytone.voice.address()?);


    Ok(())
}

fn main(){
    let src_chain = ARCHWAY_1;
    let dst_chain = JUNO_1;
    let src_admin_addr = "";
    let dst_admin_addr = "";

    instantiate_two_chains(src_chain, dst_chain, src_admin_addr, dst_admin_addr).unwrap();
}