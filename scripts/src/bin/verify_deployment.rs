use cw_orch::{prelude::networks::*, prelude::*, tokio::runtime::Runtime};
use cw_orch_interchain::prelude::*;
use cw_orch_polytone::{deploy::POLYTONE_NOTE, interchain::PolytoneConnection, PolytoneNote};
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
    let dst_daemon = DaemonBuilder::default().chain(dst_chain.clone()).build()?;

    let polytone_connection = PolytoneConnection::load_from(src_daemon.clone(), dst_daemon.clone());

    // We send an empty message on the note side
    let tx_response = polytone_connection.send_message(vec![])?;

    let interchain = DaemonInterchainEnv::from_daemons(
        rt.handle(),
        vec![src_daemon, dst_daemon],
        &ChannelCreationValidator,
    );

    interchain.check_ibc(src_chain.chain_id, tx_response)?;

    Ok(())
}
