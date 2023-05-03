use schemars::JsonSchema;
use cosmwasm_std::Storage;
use cosmwasm_std::Addr;
use cosmwasm_storage::{
    ReadonlySingleton, singleton, Singleton,
    singleton_read,
};
use secret_toolkit::storage::Keymap;

use serde::{Deserialize, Serialize};

const CONFIG_KEY: &[u8] = b"config";

pub static CANDIDATE_RESULT: Keymap<u16, u16> = Keymap::new(b"candidate_result_state");
pub static VOTERS: Keymap<Addr, bool> = Keymap::new(b"voters_state");

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub name: String,
    pub candidates_list: Vec<Candidate>,
    pub voters_addresses: Vec<Addr>,
    pub close_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Candidate {
    pub id: u16,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CandidateResult {
    id: u16,
    votes: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Voter {
    addr: Addr,
    voted: bool,
}

impl CandidateResult {
    pub fn new(id: u16, votes: u16) -> CandidateResult {
        return CandidateResult {
            id,
            votes
        }
    }
}

impl Voter {
    pub fn new(addr: Addr, voted: bool) -> Voter {
        return Voter {
            addr,
            voted
        }
    }
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
