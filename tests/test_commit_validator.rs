use serde_json::json;

use common::constants::{
    COMMIT_VALIDATOR_TIME, DEFAULT_CULTURE, DEFAULT_DEPOSIT_PROTOCOL, DEFAULT_DEPOSIT_VALIDATOR, DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_REQUEST_ID,
    DEFAULT_VALIDATOR_ANSWER, VALIDATOR_1,
};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_default_protocol_account, get_default_validator_account};

use earthmind_rs::{CommitValidatorResult, Contract, Module};

pub mod common;

#[test]
fn test_commit_by_validator_when_validator_and_request_exist() {
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

    //@dev Validator register to earthmind protocol and commit an answer to a request
    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    Environment::with_account(validator)
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    let result = contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_eq!(result, CommitValidatorResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);
}

#[test]
#[should_panic]
fn test_commit_by_validator_when_validator_dont_registered_and_request_exist() {
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

    let result = contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_eq!(result, CommitValidatorResult::Fail);

    assert_logs(vec![Log::Message("Validator is not registered: validator1.near".to_string())]);
}

#[test]
fn test_commit_by_validator_when_validator_registered_and_request_dont_exist() {
    let mut contract = Contract::new();

    let validator = get_default_validator_account();
    Environment::with_account(validator).with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR).create();

    contract.register_validator();

    let result = contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_eq!(result, CommitValidatorResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "register_validator".to_string(),
            data: vec![("validator", json![VALIDATOR_1])],
        },
        Log::Message("Request is not registered: 73ead60176d724e462dbfa8d49506177bb13bec748cf5af5019b6d1da63e204b".to_string()),
    ]);
}

#[test]
fn test_commit_by_validator_when_validator_and_request_exist_and_commit_already() {
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

    //@dev Validator register to earthmind protocol and commit an answer to a request
    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    Environment::with_account(validator)
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());
    let result = contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_eq!(result, CommitValidatorResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "commit_validator".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
        },
        Log::Message("This validator have a commit answer: validator1.near".to_string()),
    ]);
}
