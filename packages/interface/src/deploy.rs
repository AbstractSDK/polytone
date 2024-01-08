use std::path::PathBuf;

use crate::{PolytoneNote, PolytoneProxy, PolytoneVoice};
use cosmwasm_std::CosmosMsg;
use cw_orch::{
    deploy::Deploy,
    prelude::{Addr, ContractInstance, CwEnv, CwOrchError, CwOrchInstantiate, CwOrchUpload},
};
use polytone_note::msg::ExecuteMsgFns;

use crate::Polytone;

pub const POLYTONE_NOTE: &str = "polytone:note";
pub const POLYTONE_VOICE: &str = "polytone:voice";
pub const POLYTONE_PROXY: &str = "polytone:proxy";

pub const MAX_BLOCK_GAS: u64 = 100_000_000;

impl<Chain: CwEnv> Deploy<Chain> for Polytone<Chain> {
    type Error = CwOrchError;

    type DeployData = Option<String>;

    fn store_on(chain: Chain) -> Result<Self, <Self as Deploy<Chain>>::Error> {
        let polytone = Polytone::new(chain);

        polytone.note.upload()?;
        polytone.voice.upload()?;
        polytone.proxy.upload()?;

        Ok(polytone)
    }

    fn deploy_on(chain: Chain, data: Self::DeployData) -> Result<Self, CwOrchError> {
        // upload
        let deployment = Self::store_on(chain.clone())?;

        deployment.instantiate_note(data.clone())?;
        deployment.instantiate_voice(data)?;

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
                .join("state.json")
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

    pub fn instantiate_note(&self, admin: Option<String>) -> Result<Chain::Response, CwOrchError> {
        self.note.instantiate(
            &polytone_note::msg::InstantiateMsg {
                pair: None,
                block_max_gas: MAX_BLOCK_GAS.into(),
            },
            admin.map(Addr::unchecked).as_ref(),
            None,
        )
    }

    pub fn instantiate_voice(&self, admin: Option<String>) -> Result<Chain::Response, CwOrchError> {
        self.voice.instantiate(
            &polytone_voice::msg::InstantiateMsg {
                proxy_code_id: self.proxy.code_id()?.into(),
                block_max_gas: MAX_BLOCK_GAS.into(),
            },
            admin.map(Addr::unchecked).as_ref(),
            None,
        )
    }

    pub fn send_message(&self, msgs: Vec<CosmosMsg>) -> Result<Chain::Response, CwOrchError> {
        self.note.ibc_execute(msgs, 1_000_000u64.into(), None)
    }
}

#[cfg(test)]
pub mod test {
    use anyhow::Result as AnyResult;
    use cw_orch::{
        deploy::Deploy,
        prelude::{
            networks::{JUNO_1, UNI_6},
            ContractInstance, DaemonBuilder,
        },
        tokio::runtime::Runtime,
    };

    use crate::Polytone;

    /// This is a dummy mnemonic to have the daemon initialized properly
    pub const TEST_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

    #[test]
    pub fn mainnet_test() -> AnyResult<()> {
        let rt = Runtime::new()?;
        let chain = DaemonBuilder::default()
            .chain(JUNO_1)
            .handle(rt.handle())
            .mnemonic(TEST_MNEMONIC)
            .build()?;

        let polytone = Polytone::load_from(chain)?;
        polytone.note.code_id()?;
        polytone.voice.code_id()?;
        polytone.proxy.code_id()?;

        Ok(())
    }

    #[test]
    pub fn testnet_test() -> AnyResult<()> {
        let rt = Runtime::new()?;
        let chain = DaemonBuilder::default()
            .chain(UNI_6)
            .handle(rt.handle())
            .mnemonic(TEST_MNEMONIC)
            .build()?;

        let polytone = Polytone::load_from(chain)?;
        polytone.note.code_id()?;
        polytone.voice.code_id()?;
        polytone.proxy.code_id()?;

        Ok(())
    }
}
