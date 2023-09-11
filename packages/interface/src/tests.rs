use cw_orch::{prelude::Daemon, tokio::runtime::Runtime};

use crate::{Polytone, PolytoneAccount};
use cw_orch_interchain_core::{channel::IbcQueryHandler, InterchainEnv};

#[test]
fn polytone_deploy() -> AnyResult<()> {
    use cw_orch::{
        deploy::Deploy, prelude::CwOrchExecute, starship::Starship, tokio::runtime::Runtime,
    };

    env_logger::init();
    let rt = Runtime::new()?;

    let config_path = format!(
        "{}{}",
        env!("CARGO_MANIFEST_DIR"),
        "../../../../cw-orchestrator/packages/starship/examples/starship.yaml"
    );

    let starship = Starship::new(rt.handle().to_owned(), &config_path, None)?;

    let juno_polytone = Polytone::deploy_on(starship.daemon("juno-1")?.clone(), None)?;
    let stargaze_polytone = Polytone::deploy_on(starship.daemon("stargaze-1")?.clone(), None)?;

    let polytone = deploy(&rt, &starship, &juno_polytone, &stargaze_polytone)?;

    // Now we test an interaction through the interchain

    let result = polytone.source.note.execute(
        &polytone_note::msg::ExecuteMsg::Execute {
            msgs: vec![],
            callback: None,
            timeout_seconds: 1_000_000u64.into(),
        },
        None,
    )?;
    rt.block_on(
        polytone
            .channel
            .await_ibc_execution("juno-1".into(), result.txhash),
    )?;

    Ok(())
}
