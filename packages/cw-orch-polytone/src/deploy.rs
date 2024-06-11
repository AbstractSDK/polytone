use std::path::PathBuf;

use crate::{interchain::PolytoneConnection, PolytoneNote, PolytoneProxy, PolytoneVoice};
use cosmwasm_std::{CosmosMsg, IbcOrder};
use cw_orch::prelude::*;
use cw_orch_interchain::InterchainError;
use cw_orch_interchain::{IbcQueryHandler, InterchainEnv};
use polytone_note::msg::ExecuteMsgFns;

use crate::Polytone;

pub const POLYTONE_NOTE: &str = "polytone:note";
pub const POLYTONE_VOICE: &str = "polytone:voice";
pub const POLYTONE_PROXY: &str = "polytone:proxy";

pub const MAX_BLOCK_GAS: u64 = 100_000_000;

impl<Chain: CwEnv> Deploy<Chain> for Polytone<Chain> {
    type Error = CwOrchError;

    type DeployData = Empty;

    fn store_on(chain: Chain) -> Result<Self, <Self as Deploy<Chain>>::Error> {
        let polytone = Polytone::new(chain);

        polytone.note.upload()?;
        polytone.voice.upload()?;
        polytone.proxy.upload()?;

        Ok(polytone)
    }

    fn deploy_on(chain: Chain, _data: Self::DeployData) -> Result<Self, CwOrchError> {
        // Deployment of Polytone is simply uploading the contracts
        let deployment = Self::store_on(chain.clone())?;

        Ok(deployment)
    }

    fn get_contracts_mut(
        &mut self,
    ) -> Vec<Box<&mut dyn cw_orch::prelude::ContractInstance<Chain>>> {
        vec![
            Box::new(&mut self.note),
            Box::new(&mut self.voice),
            Box::new(&mut self.proxy),
        ]
    }

    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        let mut polytone = Self::new(chain);
        // We register all the contracts default state
        polytone.set_contracts_state(None);
        Ok(polytone)
    }

    fn deployed_state_file_path() -> Option<String> {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        Some(
            PathBuf::from(crate_path)
                .join("cw-orch-state.json")
                .display()
                .to_string(),
        )
    }
}

impl<Chain: CwEnv> Polytone<Chain> {
    pub fn new(chain: Chain) -> Self {
        let note = PolytoneNote::new(POLYTONE_NOTE, chain.clone());
        let voice = PolytoneVoice::new(POLYTONE_VOICE, chain.clone());
        let proxy = PolytoneProxy::new(POLYTONE_PROXY, chain.clone());

        Polytone { note, voice, proxy }
    }

    pub(crate) fn instantiate_note(
        &self,
        admin: Option<String>,
    ) -> Result<Chain::Response, CwOrchError> {
        self.note.instantiate(
            &polytone_note::msg::InstantiateMsg {
                pair: None,
                block_max_gas: MAX_BLOCK_GAS.into(),
            },
            admin.map(Addr::unchecked).as_ref(),
            None,
        )
    }

    pub(crate) fn instantiate_voice(
        &self,
        admin: Option<String>,
    ) -> Result<Chain::Response, CwOrchError> {
        self.voice.instantiate(
            &polytone_voice::msg::InstantiateMsg {
                proxy_code_id: self.proxy.code_id()?.into(),
                block_max_gas: MAX_BLOCK_GAS.into(),
            },
            admin.map(Addr::unchecked).as_ref(),
            None,
        )
    }

    pub(crate) fn instantiate(&self, admin: Option<String>) -> Result<(), CwOrchError> {
        self.instantiate_note(admin.clone())?;
        self.instantiate_voice(admin)?;
        Ok(())
    }

    pub fn send_message(&self, msgs: Vec<CosmosMsg>) -> Result<Chain::Response, CwOrchError> {
        self.note.ibc_execute(msgs, 1_000_000u64.into(), None)
    }
}
impl<Chain: CwEnv + IbcQueryHandler> Polytone<Chain> {
    pub fn connect(
        &self,
        dst: &Polytone<Chain>,
        interchain: impl InterchainEnv<Chain>,
    ) -> Result<PolytoneConnection<Chain>, InterchainError> {
        // We create a channel between the 2 polytone instances

        self.instantiate(None)?;
        dst.instantiate(None)?;

        interchain.create_contract_channel(
            &self.note,
            &dst.voice,
            "polytone-1",
            Some(IbcOrder::Unordered),
        )?;

        let polytone_connection = PolytoneConnection::load_from_deployment(self, dst)?;

        let no_addr = Addr::unchecked("no-address-registered");

        // We reset the state, this object shouldn't have registered addresses in a normal flow
        self.note.set_address(&no_addr);
        self.voice.set_address(&no_addr);
        dst.note.set_address(&no_addr);
        dst.voice.set_address(&no_addr);

        Ok(polytone_connection)
    }
}
