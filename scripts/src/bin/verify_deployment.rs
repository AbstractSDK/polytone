use cw_orch::{daemon::ChainInfo, prelude::networks::*, prelude::*, tokio::runtime::Runtime};
use cw_orch_polytone::Polytone;
use scripts::helpers::get_deployment_id;

fn main() {
    let src_chain = ARCHWAY_1;
    let dst_chain = JUNO_1;
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
    let dst_daemon = DaemonBuilder::default()
        .chain(dst_chain.clone())
        .deployment_id(deployment_id.clone())
        .handle(rt.handle())
        .build()?;
    let src_polytone = Polytone::load_from(src_daemon.clone())?;

    // We send an empty message on the note side
    let tx_response = src_polytone.send_message(vec![])?;

    let interchain = DaemonInterchainEnv::from_daemons(
        rt.handle(),
        vec![src_daemon, dst_daemon],
        &ChannelCreationValidator,
    );

    interchain.wait_ibc(&src_chain.chain_id.to_string(), tx_response)?;

    Ok(())
}
