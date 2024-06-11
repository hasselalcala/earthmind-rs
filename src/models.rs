use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::LookupMap;
use near_sdk::AccountId;

type Hash = String;

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum RegisterMinerResult {
    Success,
    AlreadyRegistered,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum RegisterValidatorResult {
    Success,
    AlreadyRegistered,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum RegisterRequestResult {
    Success,
    AlreadyRegistered,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum CommitMinerResult {
    Success,
    Fail,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum RevealMinerResult {
    Success,
    Fail,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum CommitValidatorResult {
    Success,
    Fail,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum RevealValidatorResult {
    Success,
    Fail,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum RequestState {
    NonStarted,
    CommitMiners,
    RevealMiners,
    CommitValidators,
    RevealValidators,
    Ended,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MinerProposal {
    pub proposal_hash: Hash,
    pub answer: bool,
    pub is_revealed: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ValidatorProposal {
    pub proposal_hash: Hash,
    pub is_revealed: bool,
    pub miner_addresses: Vec<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Request {
    pub sender: AccountId,
    pub request_id: String,
    pub start_time: u64,
    pub miners_proposals: LookupMap<AccountId, MinerProposal>,
    pub validators_proposals: LookupMap<AccountId, ValidatorProposal>,
    pub validators_votes_to_miner: LookupMap<AccountId, u64>,
}
