use near_workspaces::AccountId;
use serde_json::json;

use common::constants::{
    DEFAULT_CULTURE, DEFAULT_DEPOSIT_PROTOCOL, DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_MINER_ANSWER, DEFAULT_REQUEST_ID, DEFAULT_VALIDATOR_ANSWER,
};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, generate_validator_answer, get_default_miner_account, get_default_protocol_account, get_default_validator_account};

use earthmind_rs::{Contract, Module, RegisterRequestResult};

pub mod common;

#[test]
fn test_request_governance_decision_when_is_registered_returns_already_registered() {
    let mut contract = Contract::new();

    let protocol = get_default_protocol_account();
    Environment::with_account(protocol.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);

    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    let result = contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());

    assert_eq!(result, RegisterRequestResult::AlreadyRegistered);

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![protocol])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
        Log::Message("Attempted to register an already registered request: 73ead60176d724e462dbfa8d49506177bb13bec748cf5af5019b6d1da63e204b".to_string()),
    ]);
}

// Hash miner answer

#[test]
fn test_hash_miner_answer() {
    let contract = Contract::new();

    let miner = get_default_miner_account();
    Environment::with_account(miner).create();

    let request_id = DEFAULT_REQUEST_ID.to_string();
    let answer = true;
    let message = "It's a cool NFT".to_string();

    let result = contract.hash_miner_answer(request_id, answer, message);

    assert_eq!(result, DEFAULT_MINER_ANSWER);
}

// Hash validator answer
#[test]
fn test_hash_validator_answer() {
    let contract = Contract::new();

    let validator = get_default_validator_account();
    Environment::with_account(validator).create();

    let request_id = DEFAULT_REQUEST_ID.to_string();
    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    let result = contract.hash_validator_answer(request_id, answer, message);

    assert_eq!(result, DEFAULT_VALIDATOR_ANSWER);
}

#[test]
#[should_panic]
fn test_hash_validator_answer_when_answer_is_not_complete() {
    let contract = Contract::new();

    let validator = get_default_validator_account();
    Environment::with_account(validator).create();

    let request_id = DEFAULT_REQUEST_ID.to_string();
    let answer = generate_validator_answer();
    let answer: Vec<AccountId> = answer[0..answer.len() - 1].to_vec();
    let message = "It's a cool NFT".to_string();

    contract.hash_validator_answer(request_id, answer, message);
}
