use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use std::fmt;

type Hash = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    RegisterProtocol(Vec<RegisterProtocolLog>),
    RegisterMiner(Vec<RegisterMinerLog>),
    RegisterValidator(Vec<RegisterValidatorLog>),
    RegisterRequest(Vec<RegisterRequestLog>),
    CommitMiner(Vec<CommitMinerLog>),
    CommitValidator(Vec<CommitValidatorLog>),
    RevealMiner(Vec<RevealMinerLog>),
    RevealValidator(Vec<RevealValidatorLog>),
    ToptenMiners(Vec<ToptenMinersLog>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("EVENT_JSON:{}", &serde_json::to_string(self).map_err(|_| fmt::Error)?))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterProtocolLog {
    pub account: AccountId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterMinerLog {
    pub miner: AccountId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterValidatorLog {
    pub validator: AccountId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterRequestLog {
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CommitMinerLog {
    pub request_id: String,
    pub answer: Hash,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CommitValidatorLog {
    pub request_id: String,
    pub answer: Hash,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RevealMinerLog {
    pub request_id: String,
    pub answer: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RevealValidatorLog {
    pub request_id: String,
    pub answer: Vec<AccountId>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ToptenMinersLog {
    pub request_id: String,
    pub topten: Vec<(AccountId, i32)>,
}
