use near_sdk::NearToken;
use serde_json::json;

use common::constants::{DEFAULT_DEPOSIT_MINER, MINER_1, MINER_2};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_account_for_miner, get_default_miner_account};

use earthmind_rs::{Contract, RegisterMinerResult};

pub mod common;

#[test]
fn test_register_miner() {
    let miner_1 = get_default_miner_account();

    Environment::with_account(miner_1.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    let mut contract = Contract::new();
    let result_1 = contract.register_miner();

    assert_eq!(result_1, RegisterMinerResult::Success);
    assert!(contract.is_miner_registered(miner_1));

    assert_logs(vec![Log::Event {
        event_name: "register_miner".to_string(),
        data: vec![("miner", json![MINER_1])],
    }]);
}

#[test]
fn test_register_multiple_miners() {
    // register miner 1
    let miner_1 = get_default_miner_account();

    Environment::with_account(miner_1.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    let mut contract = Contract::new();
    let result_1 = contract.register_miner();

    assert_eq!(result_1, RegisterMinerResult::Success);
    assert!(contract.is_miner_registered(miner_1));

    assert_logs(vec![Log::Event {
        event_name: "register_miner".to_string(),
        data: vec![("miner", json![MINER_1])],
    }]);

    // register miner 2
    let miner_2: near_sdk::AccountId = get_account_for_miner(MINER_2);

    Environment::with_account(miner_2.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
    let result_2 = contract.register_miner();

    assert_eq!(result_2, RegisterMinerResult::Success);
    assert!(contract.is_miner_registered(miner_2));

    assert_logs(vec![Log::Event {
        event_name: "register_miner".to_string(),
        data: vec![("miner", json![MINER_2])],
    }]);
}

#[test]
fn test_register_miner_when_is_registered_returns_already_registered() {
    let miner_1 = get_default_miner_account();

    Environment::with_account(miner_1).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    let mut contract = Contract::new();
    contract.register_miner();

    let result = contract.register_miner();

    assert_eq!(result, RegisterMinerResult::AlreadyRegistered);

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![MINER_1])],
        },
        Log::Message("Attempted to register an already registered miner: miner1.near".to_string()),
    ]);
}

#[test]
#[should_panic]
fn test_register_miner_when_deposit_is_less_min_stake() {
    let miner_1 = get_default_miner_account();
    let mut contract = Contract::new();

    let register_deposit = NearToken::from_yoctonear(10u128.pow(23));
    Environment::with_account(miner_1).with_attached_deposit(register_deposit).create();

    contract.register_miner();
}

#[test]
fn test_is_miner_registered() {
    let miner_1: near_sdk::AccountId = get_default_miner_account();

    Environment::with_account(miner_1.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();

    let mut contract = Contract::new();
    contract.register_miner();

    assert!(contract.is_miner_registered(miner_1));
}

#[test]
fn test_is_miner_registered_when_not_registered() {
    let contract = Contract::new();

    let miner_1: near_sdk::AccountId = get_default_miner_account();

    assert!(!contract.is_miner_registered(miner_1));
}
