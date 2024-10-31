use common::constants::{
    DEFAULT_CULTURE, DEFAULT_DEPOSIT_MINER, DEFAULT_DEPOSIT_PROTOCOL, DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_MINER_ANSWER, DEFAULT_REQUEST_ID, MINER_1, MINER_2,
    REVEAL_MINER_TIME,
};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_account_for_miner, get_default_miner_account, get_default_protocol_account};

use earthmind_rs::{Contract, Module, RevealMinerResult};

use serde_json::json;

pub mod common;

#[test]
fn test_reveal_by_miner() {
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

    Environment::with_account(miner.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    contract.register_miner();
    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

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

    Environment::with_account(miner).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = true;
    let message = "It's a cool NFT";

    let result = contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.to_string());

    assert_eq!(result, RevealMinerResult::Success);

    assert_logs(vec![Log::Event {
        event_name: "reveal_miner".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);
}

#[test]
fn test_reveal_by_miner_when_miner_is_not_registered() {
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

    let miner_1 = get_default_miner_account();

    Environment::with_account(miner_1).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    contract.register_miner();

    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

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

    let miner_2 = get_account_for_miner(MINER_2);
    Environment::with_account(miner_2).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = true;
    let message = "It's a cool NFT";

    let result = contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.to_string());

    assert_eq!(result, RevealMinerResult::Fail);

    assert_logs(vec![Log::Message("Miner not registered: miner2.near".to_string())]);
}

#[test]
fn test_reveal_by_miner_when_request_is_not_registered() {
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

    let miner_1 = get_default_miner_account();

    Environment::with_account(miner_1).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    contract.register_miner();

    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

    let fail_request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();
    let answer = true;
    let message = "It's a cool NFT".to_string();

    let result = contract.reveal_by_miner(fail_request_id, answer, message);

    assert_eq!(result, RevealMinerResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_MINER_ANSWER])],
        },
        Log::Message("Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string()),
    ]);
}

#[test]
fn test_reveal_by_miner_when_proposal_is_already_reveal() {
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

    let miner_1 = get_default_miner_account();

    Environment::with_account(miner_1.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    contract.register_miner();

    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

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

    Environment::with_account(miner_1).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = true;
    let message = "It's a cool NFT".to_string();

    contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.clone());

    let result = contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.clone());

    assert_eq!(result, RevealMinerResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "reveal_miner".to_string(),
            data: vec![
                ("request_id", json![DEFAULT_REQUEST_ID]),
                ("answer", json![answer]),
                ("message", json![message]),
            ],
        },
        Log::Message("Proposal already revealed".to_string()),
    ]);
}

#[test]
fn test_reveal_by_miner_when_answer_not_equal() {
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

    let miner_1 = get_default_miner_account();

    Environment::with_account(miner_1.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    contract.register_miner();

    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), DEFAULT_MINER_ANSWER.to_string());

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

    Environment::with_account(miner_1).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = false;
    let message = "It's a cool NFT";
    let result = contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.to_string());

    assert_eq!(result, RevealMinerResult::Fail);
    assert_logs(vec![Log::Message("Answer don't match".to_string())]);
}
