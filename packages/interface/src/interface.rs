use cw_orch::prelude::*;
use polytone_note::contract::PolytoneNote;
use polytone_proxy::contract::PolytoneProxy;
use polytone_voice::contract::PolytoneVoice;

use cw_orch_interchain_core::channel::IbcQueryHandler;
use cw_orch_interchain_core::InterchainEnv;

pub const POLYTONE_VERSION: &str = "polytone-1";

// This file contains all interfaces to the polytone contracts
pub struct PolytoneAccount<Chain: CwEnv> {
    pub source: Polytone<Chain>,
    pub dest: Polytone<Chain>,
}

impl<Chain: IbcQueryHandler> PolytoneAccount<Chain> {
    // This is purely for testing fow, not in production
    pub async fn create<IBC: InterchainEnv<Chain>>(
        interchain_env: &IBC,
        source: &Polytone<Chain>,
        dest: &Polytone<Chain>,
    ) -> anyhow::Result<PolytoneAccount<Chain>> {
        // We need to create a channel between the two contracts
        interchain_env
            .create_contract_channel(&source.note, &dest.voice, None, POLYTONE_VERSION)
            .await?;

        Ok(PolytoneAccount {
            source: source.clone(),
            dest: dest.clone(),
        })
    }

    pub fn load_from(source: &Polytone<Chain>, dest: &Polytone<Chain>) -> PolytoneAccount<Chain> {
        PolytoneAccount {
            source: source.clone(),
            dest: dest.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Polytone<Chain: CwEnv> {
    pub note: PolytoneNote<Chain>,
    pub voice: PolytoneVoice<Chain>,
    pub proxy: PolytoneProxy<Chain>, // This contract doesn't have an address, it's only a code id  used for instantiating
}