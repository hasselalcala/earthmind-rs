use near_sdk::test_utils::get_logs;
use serde_json::json;

use common::constants::{
    DEFAULT_CULTURE, DEFAULT_DEPOSIT_MINER, DEFAULT_DEPOSIT_PROTOCOL, DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_MINER_ANSWER, DEFAULT_REQUEST_ID, MINER_1,
};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_default_miner_account, get_default_protocol_account};

use earthmind_rs::{CommitMinerResult, Contract, Module};

pub mod common;

#[test]
fn test_commit_by_miner_when_miner_and_request_exist() {
    let mut contract = Contract::new();

    // @dev Protocol register to earthmind protocol and request a governance decision
    let protocol = get_default_protocol_account();
    Environment::with_account(protocol.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());
    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![protocol])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
    ]);

    // @dev Miner register to earthmind protocol
    let miner = get_default_miner_account();
    Environment::with_account(miner).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    contract.register_miner();

    let result = contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_eq!(result, CommitMinerResult::Success);

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_MINER_ANSWER])],
        },
    ]);
}

#[test]
fn test_commit_by_miner_when_miner_dont_registered_and_request_exist() {
    let mut contract = Contract::new();

    // @dev Protocol register to earthmind protocol and request a governance decision
    let protocol = get_default_protocol_account();
    Environment::with_account(protocol.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());
    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![protocol])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
    ]);

    let miner = get_default_miner_account();
    Environment::with_account(miner).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    let result = contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_eq!(result, CommitMinerResult::Fail);

    let logs = get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Miner not registered: miner1.near");
}

#[test]
fn test_commit_by_miner_when_miner_registered_and_request_dont_exist() {
    let mut contract = Contract::new();

    // @dev Protocol register to earthmind protocol and request a governance decision
    let protocol = get_default_protocol_account();
    Environment::with_account(protocol.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);

    assert_logs(vec![Log::Event {
        event_name: "register_protocol".to_string(),
        data: vec![("account", json![protocol])],
    }]);

    let miner = get_default_miner_account();
    Environment::with_account(miner).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    contract.register_miner();

    let result = contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_eq!(result, CommitMinerResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Message("Request is not registered: 73ead60176d724e462dbfa8d49506177bb13bec748cf5af5019b6d1da63e204b".to_string()),
    ]);
}

#[test]
fn test_commit_by_miner_when_miner_and_request_exist_and_commit_already() {
    let mut contract = Contract::new();

    // @dev Protocol register to earthmind protocol and request a governance decision
    let protocol = get_default_protocol_account();
    Environment::with_account(protocol.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);
    contract.request_governance_decision(DEFAULT_MESSAGE_TO_REQUEST.to_string());
    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![protocol])],
        },
        Log::Event {
            event_name: "register_request".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID])],
        },
    ]);

    let miner = get_default_miner_account();
    Environment::with_account(miner).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    contract.register_miner();

    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    let result = contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    assert_eq!(result, CommitMinerResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_MINER_ANSWER])],
        },
        Log::Message("This miner have a commit answer: miner1.near".to_string()),
    ]);
}
