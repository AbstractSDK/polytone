use cosmos_sdk_proto::cosmwasm::wasm::v1::MsgClearAdminResponse;
use cosmos_sdk_proto::traits::Message;
use cosmos_sdk_proto::Any;
use cosmos_sdk_proto::{cosmwasm::wasm::v1::MsgClearAdmin, traits::TypeUrl};
use cw_orch::daemon::networks::{JUNO_1, NEUTRON_1, OSMOSIS_1, PHOENIX_1};
use cw_orch::daemon::Daemon;
use cw_orch::state::{ChainState, StateInterface};
use cw_orch::{
    daemon::{networks::ARCHWAY_1, ChainInfo, DaemonBuilder},
    environment::TxHandler,
    prelude::Stargate,
    tokio::runtime::Runtime,
};
use scripts::helpers::get_deployment_id;
use scripts::STARGAZE_1;

pub fn main() {
    dotenv::dotenv().unwrap();
    pretty_env_logger::init();
    let mut juno = JUNO_1;
    juno.grpc_urls = &["http://juno-grpc.polkachu.com:12690"];
    let all_chains = &[/*juno, OSMOSIS_1,  NEUTRON_1,*/ STARGAZE_1, PHOENIX_1];
    for chain in all_chains {
        change_admins(&ARCHWAY_1, chain).unwrap();
        change_admins(chain, &ARCHWAY_1).unwrap();
    }
}

pub fn change_admins(src_chain: &ChainInfo, dst_chain: &ChainInfo) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let deployment_id = get_deployment_id(src_chain, dst_chain);
    let src_daemon = DaemonBuilder::default()
        .handle(rt.handle())
        .deployment_id(deployment_id.clone())
        .chain(src_chain.clone())
        .build()?;
    let dst_daemon = DaemonBuilder::default()
        .handle(rt.handle())
        .deployment_id(deployment_id.clone())
        .chain(dst_chain.clone())
        .build()?;

    change_one_chain_admin(&src_daemon)?;
    change_one_chain_admin(&dst_daemon)?;

    Ok(())
}

pub fn change_one_chain_admin(daemon: &Daemon) -> anyhow::Result<()> {
    let addresses = daemon.state().get_all_addresses()?;
    daemon.commit_any::<MsgClearAdminResponse>(
        addresses
            .values()
            .map(|a| Any {
                type_url: MsgClearAdmin::TYPE_URL.to_string(),
                value: MsgClearAdmin {
                    sender: daemon.sender().to_string(),
                    contract: a.to_string(),
                }
                .encode_to_vec(),
            })
            .collect(),
        None,
    )?;

    Ok(())
}
