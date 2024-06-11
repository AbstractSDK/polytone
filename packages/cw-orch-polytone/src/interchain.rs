use cosmwasm_std::CosmosMsg;
use cw_orch::prelude::*;
use polytone_note::msg::ExecuteMsgFns;

use crate::{
    deploy::{POLYTONE_NOTE, POLYTONE_PROXY, POLYTONE_VOICE},
    Polytone, PolytoneNote, PolytoneProxy, PolytoneVoice,
};

/// Represents an Polytone connection
///
/// The note contract is on the local chain while the voice and proxy contracts are located on the remote chain
#[derive(Clone)]
pub struct PolytoneConnection<Chain: CwEnv> {
    pub note: PolytoneNote<Chain>,
    pub voice: PolytoneVoice<Chain>,
    pub proxy: PolytoneProxy<Chain>, // This contract doesn't have an address, it's only a code id  used for instantiating
}

impl<Chain: CwEnv> PolytoneConnection<Chain> {
    pub fn load_from(src_chain: Chain, dst_chain: Chain) -> PolytoneConnection<Chain> {
        let suffix = connection_suffix(&src_chain, &dst_chain);

        PolytoneConnection {
            note: PolytoneNote::new(format!("{}:{}", POLYTONE_NOTE, suffix), src_chain),
            voice: PolytoneVoice::new(format!("{}:{}", POLYTONE_VOICE, suffix), dst_chain.clone()),
            // Proxy doesn't have a specific address in deployments, so we don't load a specific suffixed proxy
            proxy: PolytoneProxy::new(POLYTONE_PROXY, dst_chain),
        }
    }

    pub(crate) fn load_from_deployment(
        src_polytone: &Polytone<Chain>,
        dst_polytone: &Polytone<Chain>,
    ) -> Result<PolytoneConnection<Chain>, CwOrchError> {
        let connection = Self::load_from(
            src_polytone.note.get_chain().clone(),
            dst_polytone.voice.get_chain().clone(),
        );

        connection.note.set_address(&src_polytone.note.address()?);
        connection.voice.set_address(&dst_polytone.voice.address()?);

        Ok(connection)
    }

    pub fn send_message(&self, msgs: Vec<CosmosMsg>) -> Result<Chain::Response, CwOrchError> {
        self.note.ibc_execute(msgs, 1_000_000u64.into(), None)
    }

    pub fn connection_suffix(&self) -> String {
        connection_suffix(self.note.get_chain(), self.voice.get_chain())
    }
}

pub fn connection_suffix<Chain: CwEnv>(src_chain: &Chain, dst_chain: &Chain) -> String {
    format!(
        "{}-->{}",
        src_chain.env_info().chain_id,
        dst_chain.env_info().chain_id,
    )
}
