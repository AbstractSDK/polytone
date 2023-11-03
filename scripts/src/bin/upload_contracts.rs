use cw_orch::{prelude::*, prelude::networks::ARCHWAY_1, tokio::runtime::Runtime, daemon::ChainInfo};
use cw_orch_polytone::Polytone;


fn main(){   
    let chain = ARCHWAY_1;
    upload_contracts(chain).unwrap();
}

fn upload_contracts(chain: ChainInfo) -> anyhow::Result<()>{

    pretty_env_logger::init();
    dotenv::dotenv()?;
    let rt = Runtime::new()?;
    let daemon = DaemonBuilder::default()
        .chain(chain)
        .handle(rt.handle())
        .build()?;

    let _polytone = Polytone::store_on(daemon)?;

    Ok(())
}
