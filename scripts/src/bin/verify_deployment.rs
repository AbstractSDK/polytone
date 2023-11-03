use cw_orch::{
    daemon::ChainInfo,
    prelude::networks::{osmosis::OSMOSIS_1, ARCHWAY_1},
    prelude::*,
    tokio::runtime::Runtime,
};
use cw_orch_polytone::Polytone;
use scripts::helpers::get_deployment_id;

fn main() {
    let src_chain = ARCHWAY_1;
    let dst_chain = OSMOSIS_1;
    verify_deployment(src_chain, dst_chain).unwrap();
}

fn verify_deployment(src_chain: ChainInfo, dst_chain: ChainInfo) -> anyhow::Result<()> {
    pretty_env_logger::init();
    dotenv::dotenv()?;
    let rt = Runtime::new()?;

    let deployment_id = get_deployment_id(&src_chain, &dst_chain);
    let src_daemon = DaemonBuilder::default()
        .chain(src_chain.clone())
        .deployment_id(deployment_id.clone())
        .handle(rt.handle())
        .build()?;
    let src_polytone = Polytone::load_from(src_daemon)?;

    // We send an empty message on the note side
    let tx_response = src_polytone.send_message(vec![])?;
    log::info!(
        "Packet successfuly brodacsted on {} tx : {}",
        src_chain.chain_id,
        tx_response.txhash
    );

    Ok(())
}
