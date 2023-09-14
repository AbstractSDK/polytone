use crate::interface::PolytoneConnection;
use crate::Polytone;
use cw_orch::prelude::*;
use cw_orch::{deploy::Deploy, prelude::CwOrchExecute, tokio::runtime::Runtime};

/// This allows env_logger to start properly for tests
/// The logs will be printed only if the test fails !
pub fn logger_test_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn ibc_deploy_helper<Chain: IbcQueryHandler, IBC: InterchainEnv<Chain>>(
    runtime: &Runtime,
    interchain: &IBC,
    src_chain: &str,
    dst_chain: &str,
) -> anyhow::Result<()> {
    let juno_polytone = Polytone::deploy_on(interchain.chain(src_chain)?.clone(), None)?;
    let stargaze_polytone = Polytone::deploy_on(interchain.chain(dst_chain)?.clone(), None)?;

    let polytone = runtime.block_on(PolytoneConnection::connect(
        interchain,
        &juno_polytone,
        &stargaze_polytone,
    ))?;

    // Now we test an interaction through the interchain

    let result = polytone.source.note.execute(
        &polytone_note::msg::ExecuteMsg::Execute {
            msgs: vec![],
            callback: None,
            timeout_seconds: 1_000_000u64.into(),
        },
        None,
    )?;
    runtime.block_on(interchain.wait_ibc(&src_chain.to_string(), result))?;
    Ok(())
}

#[test]
fn polytone_deploy_starship() -> anyhow::Result<()> {
    logger_test_init();
    let rt = Runtime::new()?;

    let starship = Starship::new(rt.handle().to_owned(), None)?;

    let interchain = starship.interchain_env();

    ibc_deploy_helper(&rt, &interchain, "juno-1", "stargaze-1")?;

    Ok(())
}

#[test]
fn polytone_deploy_mock() -> anyhow::Result<()> {
    logger_test_init();
    let rt = Runtime::new()?;

    let sender = Addr::unchecked("sender");

    let interchain = MockInterchainEnv::new(vec![("juno-1", &sender), ("stargaze-1", &sender)]);

    ibc_deploy_helper(&rt, &interchain, "juno-1", "stargaze-1")?;

    Ok(())
}
