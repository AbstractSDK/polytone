use cw_orch::{daemon::ChainInfo, prelude::networks::*, prelude::*, tokio::runtime::Runtime};
use cw_orch_polytone::Polytone;

fn main() {
    let mut chain = NEUTRON_1;
    chain.gas_price = 0.56;
    upload_contracts(chain).unwrap();
}

fn upload_contracts(chain: ChainInfo) -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let rt = Runtime::new()?;
    let daemon = DaemonBuilder::default()
        .chain(chain)
        .handle(rt.handle())
        .build()?;

    let _polytone = Polytone::store_on(daemon)?;

    Ok(())
}
