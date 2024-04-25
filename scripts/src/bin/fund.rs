use std::env;

use cosmwasm_std::coin;
use cw_orch::daemon::networks::{JUNO_1, NEUTRON_1, PHOENIX_1};
use cw_orch::prelude::*;
use cw_orch::{daemon::DaemonBuilder, environment::TxHandler, tokio::runtime::Runtime};
use scripts::STARGAZE_1;
pub fn main() {
    dotenv::dotenv().unwrap();
    pretty_env_logger::init();
    let mut juno = JUNO_1;
    juno.grpc_urls = &["http://juno-grpc.polkachu.com:12690"];
    let all_chains = &[
        /*ARCHWAY_1, juno, OSMOSIS_1, */ NEUTRON_1, STARGAZE_1, PHOENIX_1,
    ];
    let all_funds = &[
        // coin(10_000_000_000_000_000_000, "aarch"),
        // coin(1_000_000, "ujuno"),
        // coin(1_000_000, "uosmo"),
        coin(1_000_000, "untrn"),
        coin(1_000_000, "ustars"),
        coin(1_000_000, "uluna"),
    ];
    for (chain, funds) in all_chains.iter().zip(all_funds) {
        fund_admin_wallet(chain, funds.clone()).unwrap();
    }
}

pub fn fund_admin_wallet(chain: &ChainInfo, funds: Coin) -> anyhow::Result<()> {
    let rt: Runtime = Runtime::new()?;
    let wallet_to_fund = DaemonBuilder::default()
        .handle(rt.handle())
        .chain(chain.clone())
        .build()?;
    let mainnet_wallet = DaemonBuilder::default()
        .handle(rt.handle())
        .mnemonic(env::var("MAIN_DEPLOYMENT_MNEMONIC")?)
        .chain(chain.clone())
        .build()?;

    rt.block_on(
        mainnet_wallet
            .daemon
            .sender
            .bank_send(wallet_to_fund.sender().as_ref(), vec![funds]),
    )?;

    Ok(())
}