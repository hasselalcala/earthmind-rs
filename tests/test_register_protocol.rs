use near_sdk::NearToken;
use serde_json::json;

use common::constants::{ACCOUNT_1, ACCOUNT_2, DEFAULT_CULTURE, DEFAULT_DEPOSIT_PROTOCOL};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_account_for_protocol, get_default_protocol_account};

use earthmind_rs::{Contract, Module, RegisterProtocolResult};

pub mod common;

#[test]
fn test_register_protocol() {
    let account_1 = get_default_protocol_account();

    Environment::with_account(account_1.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let mut contract = Contract::new();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    let result_1 = contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);

    assert_eq!(result_1, RegisterProtocolResult::Success);
    assert!(contract.is_protocol_registered(account_1));

    assert_logs(vec![Log::Event {
        event_name: "register_protocol".to_string(),
        data: vec![("account", json![ACCOUNT_1])],
    }]);
}

#[test]
fn test_register_multiple_protocols() {
    // register account 1
    let account_1 = get_default_protocol_account();

    Environment::with_account(account_1.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let mut contract = Contract::new();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    let result_1 = contract.register_protocol(DEFAULT_CULTURE.to_string(), modules.clone());

    assert_eq!(result_1, RegisterProtocolResult::Success);
    assert!(contract.is_protocol_registered(account_1));

    assert_logs(vec![Log::Event {
        event_name: "register_protocol".to_string(),
        data: vec![("account", json![ACCOUNT_1])],
    }]);
    // register account 2
    let account_2: near_sdk::AccountId = get_account_for_protocol(ACCOUNT_2);

    Environment::with_account(account_2.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let result_2 = contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);

    assert_eq!(result_2, RegisterProtocolResult::Success);
    assert!(contract.is_protocol_registered(account_2));

    assert_logs(vec![Log::Event {
        event_name: "register_protocol".to_string(),
        data: vec![("account", json![ACCOUNT_2])],
    }]);
}

#[test]
fn test_register_protocol_when_is_registered_returns_already_registered() {
    let account_1 = get_default_protocol_account();

    Environment::with_account(account_1).with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL).create();

    let mut contract = Contract::new();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules.clone());

    let result = contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);

    assert_eq!(result, RegisterProtocolResult::AlreadyRegistered);

    assert_logs(vec![
        Log::Event {
            event_name: "register_protocol".to_string(),
            data: vec![("account", json![ACCOUNT_1])],
        },
        Log::Message("Attempted to register an already registered account: account1.near".to_string()),
    ]);
}

#[test]
#[should_panic]
fn test_register_protocol_when_deposit_is_less_min_stake() {
    let acoount_1 = get_default_protocol_account();
    let deposit = NearToken::from_yoctonear(10u128.pow(23));

    Environment::with_account(acoount_1).with_attached_deposit(deposit).create();

    let mut contract = Contract::new();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);
}

#[test]
fn test_is_protocol_registered() {
    let account_1: near_sdk::AccountId = get_default_protocol_account();

    Environment::with_account(account_1.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_PROTOCOL)
        .create();

    let mut contract = Contract::new();

    let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
    contract.register_protocol(DEFAULT_CULTURE.to_string(), modules);

    assert!(contract.is_protocol_registered(account_1));
}

#[test]
fn test_is_protocol_registered_when_not_registered() {
    let contract = Contract::new();

    let account_1: near_sdk::AccountId = get_default_protocol_account();

    assert!(!contract.is_protocol_registered(account_1));
}
