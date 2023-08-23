// Start by uploading the voice, note and proxy contract

// Then instantiate the voice and note

use std::str::FromStr;

use anyhow::Result as AnyResult;

use cw_orch::{
    prelude::{
        interchain_channel_builder::InterchainChannelBuilder,
        CwOrchQuery, Daemon
    },
    starship::Starship,
    tokio::runtime::Runtime,
};
use ibc_relayer_types::core::ics24_host::identifier::ChannelId;

use crate::{PolytoneAccount, Polytone};

pub const POLYTONE_VERSION: &str = "polytone-1";

// This is to be used with starship only. This is purely for testing
pub fn deploy(
    rt: &Runtime,
    starship: &Starship,
    source: &Polytone<Daemon>,
    dest:  &Polytone<Daemon>,
) -> AnyResult<PolytoneAccount<Daemon>> {

    // We need to create a channel between the two contracts
    let interchain_channel = rt.block_on(
        InterchainChannelBuilder::default()
            .from_contracts(&source.note, &dest.voice)
            .create_channel(starship.client(), POLYTONE_VERSION),
    )?;

    Ok(PolytoneAccount {
        source: source.clone(),
        dest: dest.clone(),
        channel: interchain_channel,
    })
}

pub fn load_from(
    rt: &Runtime,
    source: &Polytone<Daemon>,
    dest:  &Polytone<Daemon>,
) -> AnyResult<PolytoneAccount<Daemon>> {

    // We need to get the interchain channel object to spot the channel 
    let channel: Option<String> = source.note.query(&polytone_note::msg::QueryMsg::ActiveChannel {})?;

    let interchain_channel = rt.block_on(
        InterchainChannelBuilder::default()
            .from_contracts(&source.note, &dest.voice)
            .channel_from(ChannelId::from_str(channel.unwrap().as_str())?),
    )?;


    Ok(PolytoneAccount {
        source: source.clone(),
        dest: dest.clone(),
        channel: interchain_channel,
    })
}

#[test]
fn polytone_deploy() -> AnyResult<()> {
    use cw_orch::{
        prelude::CwOrchExecute,
        starship::Starship,
        tokio::runtime::Runtime,
        deploy::Deploy,
    };


    env_logger::init();
    let rt = Runtime::new()?;


    let config_path = format!(
        "{}{}",
        env!("CARGO_MANIFEST_DIR"),
        "../../../../cw-orchestrator/packages/starship/examples/starship.yaml"
    );

    let starship = Starship::new(rt.handle().to_owned(),&config_path, None)?;

    let juno_polytone = Polytone::deploy_on(starship.daemon("juno-1")?.clone(),None)?;
    let stargaze_polytone = Polytone::deploy_on(starship.daemon("stargaze-1")?.clone(),None)?;

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
