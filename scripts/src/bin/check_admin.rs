use anyhow::bail;
use cw_orch::daemon::networks::{JUNO_1, NEUTRON_1, OSMOSIS_1, PHOENIX_1};
use cw_orch::daemon::queriers::CosmWasm;
use cw_orch::daemon::Daemon;
use cw_orch::environment::ChainState;
use cw_orch::{
    daemon::{networks::ARCHWAY_1, DaemonBuilder},
    prelude::*,
    tokio::runtime::Runtime,
};
use scripts::helpers::get_deployment_id;
use scripts::STARGAZE_1;

pub fn main() {
    dotenv::dotenv().unwrap();
    pretty_env_logger::init();
    let mut juno = JUNO_1;
    juno.grpc_urls = &["http://juno-grpc.polkachu.com:12690"];
    let all_chains = &[juno, OSMOSIS_1, NEUTRON_1, STARGAZE_1, PHOENIX_1];
    for chain in all_chains {
        check_admins(&ARCHWAY_1, chain).unwrap();
        check_admins(chain, &ARCHWAY_1).unwrap();
    }
}

pub fn check_admins(src_chain: &ChainInfo, dst_chain: &ChainInfo) -> anyhow::Result<()> {
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

    check_one_chain_admin(&src_daemon)?;
    check_one_chain_admin(&dst_daemon)?;

    Ok(())
}

pub fn check_one_chain_admin(daemon: &Daemon) -> anyhow::Result<()> {
    let addresses = daemon.state().get_all_addresses()?;

    let wasm = CosmWasm::new_async(daemon.channel());

    addresses
        .values()
        .map(|a| {
            let contract_info = daemon.rt_handle.block_on(wasm._contract_info(a))?;
            if !contract_info.admin.is_empty() {
                bail!("{:?} admin is non empty", a);
            }

            Ok(())
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}
