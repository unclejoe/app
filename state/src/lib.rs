#![no_std]

use app_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn pingers(state: State) -> Vec<ActorId> {
        state.into_iter().map(|(pinger, _)| pinger).collect()
    }

    pub fn ping_count(state: State, actor: ActorId) -> u128 {
        state
            .into_iter()
            .find_map(|(pinger, ping_count)| (pinger == actor).then_some(ping_count))
            .unwrap_or_default()
    }

    pub fn counts(state: State) -> u128 {
        let mut counts = 0;
        state
            .into_iter()
            .for_each(|(_, count)| counts = counts + count);
        counts
    }
}
