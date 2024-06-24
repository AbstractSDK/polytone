use cw_orch::environment::ChainState;
use cw_orch::{prelude::networks::*, prelude::*, tokio::runtime::Runtime};
use cw_orch_interchain::prelude::*;
use cw_orch_polytone::interchain::PolytoneConnection;
fn main() {
    let src_chain = XION_TESTNET_1;
    let dst_chain = PION_1;
    verify_deployment(src_chain, dst_chain).unwrap();
}

fn verify_deployment(src_chain: ChainInfo, dst_chain: ChainInfo) -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let rt = Runtime::new()?;

    let src_daemon = DaemonBuilder::default().chain(src_chain.clone()).build()?;
    let dst_daemon = DaemonBuilder::default()
        .chain(dst_chain.clone())
        .state(src_daemon.state())
        .build()?;

    let interchain = DaemonInterchainEnv::from_daemons(
        rt.handle(),
        vec![src_daemon, dst_daemon],
        &ChannelCreationValidator,
    );

    // let polytone_connection = PolytoneConnection::load_from(src_daemon.clone(), dst_daemon.clone());
    let polytone_connection = PolytoneConnection::deploy_between_if_needed(
        &interchain,
        src_chain.chain_id,
        dst_chain.chain_id,
    )?;

    // We send an empty message on the note side
    let tx_response = polytone_connection.send_message(vec![])?;

    interchain.check_ibc(src_chain.chain_id, tx_response)?;

    Ok(())
}
