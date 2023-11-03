use cw_orch::{prelude::*, prelude::networks::ARCHWAY_1, tokio::runtime::Runtime, daemon::ChainInfo};
use cw_orch_polytone::Polytone;


fn main(){   
    let chain = ARCHWAY_1;
    verify_deployment(chain).unwrap();
}

fn verify_deployment(chain: ChainInfo) -> anyhow::Result<()>{

    pretty_env_logger::init();
    dotenv::dotenv()?;
    let rt = Runtime::new()?;
    let daemon = DaemonBuilder::default()
        .chain(chain.clone())
        .handle(rt.handle())
        .build()?;

    let polytone = Polytone::load_from(daemon)?;
    println!("Note code id on {} : {}", chain.chain_id, polytone.note.code_id()?);

    Ok(())
}
