use near_sdk::AccountId;
use serde_json::json;

use common::constants::{DEFAULT_DEPOSIT_VALIDATOR, VALIDATOR_1, VALIDATOR_2};
use common::environment::Environment;
use common::types::Log;
use common::utils::{assert_logs, get_account_for_validator, get_default_validator_account};

use earthmind_rs::{Contract, RegisterValidatorResult};

pub mod common;

#[test]
fn test_register_validator() {
    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    let mut contract = Contract::new();
    let result_1 = contract.register_validator();
    assert_eq!(result_1, RegisterValidatorResult::Success);
    assert!(contract.is_validator_registered(validator));

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);
}

#[test]
fn test_register_multiple_validators() {
    // register validator 1
    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    let mut contract = Contract::new();

    let result_1 = contract.register_validator();
    assert_eq!(result_1, RegisterValidatorResult::Success);
    assert!(contract.is_validator_registered(validator));

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    //register validator 2
    let validator = get_account_for_validator(VALIDATOR_2);

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    let result_2 = contract.register_validator();
    assert_eq!(result_2, RegisterValidatorResult::Success);
    assert!(contract.is_validator_registered(validator));

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_2])],
    }]);
}

#[test]
fn test_register_validator_when_is_registered_returns_already_registered() {
    let validator = get_default_validator_account();

    Environment::with_account(validator).with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR).create();

    let mut contract = Contract::new();
    contract.register_validator();

    let result = contract.register_validator();
    assert_eq!(result, RegisterValidatorResult::AlreadyRegistered);

    assert_logs(vec![
        Log::Event {
            event_name: "register_validator".to_string(),
            data: vec![("validator", json![VALIDATOR_1])],
        },
        Log::Message("Attempted to register an already registered validator: validator1.near".to_string()),
    ]);
}

#[test]
#[should_panic]
fn test_register_validator_when_deposit_is_less_min_stake() {
    let validator = get_default_validator_account();
    Environment::with_account(validator).create();

    let mut contract = Contract::new();

    contract.register_validator();
}

#[test]
fn test_is_validator_registered() {
    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();
    let mut contract = Contract::new();
    contract.register_validator();

    assert!(contract.is_validator_registered(validator));
}

#[test]
fn test_is_validator_registered_when_not_registered() {
    let contract = Contract::new();
    let validator: AccountId = get_default_validator_account();

    assert!(!contract.is_validator_registered(validator));
}
