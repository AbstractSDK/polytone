use cw_orch::{interchain::interchain_channel::InterchainChannel, prelude::*};
use polytone_note::contract::PolytoneNote;
use polytone_proxy::contract::PolytoneProxy;
use polytone_voice::contract::PolytoneVoice;
// This file contains all interfaces to the polytone contracts

pub struct PolytoneAccount<Chain: CwEnv> {
    pub source: Polytone<Chain>,
    pub dest: Polytone<Chain>,
    pub channel: InterchainChannel,
}

#[derive(Clone)]
pub struct Polytone<Chain: CwEnv> {
    pub note: PolytoneNote<Chain>,
    pub voice: PolytoneVoice<Chain>,
    pub proxy: PolytoneProxy<Chain>, // This contract doesn't have an address, it's only a code id  used for instantiating
}
