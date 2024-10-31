use super::{
    constants::{DEFAULT_MINER_ACCOUNT_ID, DEFAULT_PROTOCOL_ACCOUNT_ID, DEFAULT_VALIDATOR_ACCOUNT_ID},
    types::Log,
};
use near_sdk::{test_utils::get_logs, AccountId};
use serde_json::{json, Value};

pub fn get_account_for_protocol(account: &str) -> AccountId {
    account.parse().unwrap()
}

pub fn get_default_protocol_account() -> AccountId {
    DEFAULT_PROTOCOL_ACCOUNT_ID.parse().unwrap()
}

pub fn get_account_for_miner(miner: &str) -> AccountId {
    miner.parse().unwrap()
}

pub fn get_default_miner_account() -> AccountId {
    DEFAULT_MINER_ACCOUNT_ID.parse().unwrap()
}

pub fn get_account_for_validator(validator: &str) -> AccountId {
    validator.parse().unwrap()
}

pub fn get_default_validator_account() -> AccountId {
    DEFAULT_VALIDATOR_ACCOUNT_ID.parse().unwrap()
}
pub fn generate_validator_answer() -> Vec<AccountId> {
    let value = vec![
        "miner1.near".parse().unwrap(),
        "miner2.near".parse().unwrap(),
        "miner3.near".parse().unwrap(),
        "miner4.near".parse().unwrap(),
        "miner5.near".parse().unwrap(),
        "miner6.near".parse().unwrap(),
        "miner7.near".parse().unwrap(),
        "miner8.near".parse().unwrap(),
        "miner9.near".parse().unwrap(),
        "miner10.near".parse().unwrap(),
    ];
    value
}

pub fn group_registered_miners() -> Vec<AccountId> {
    let value = vec![
        "miner1.near".parse().unwrap(),
        "miner2.near".parse().unwrap(),
        "miner3.near".parse().unwrap(),
        "miner4.near".parse().unwrap(),
        "miner5.near".parse().unwrap(),
        "miner6.near".parse().unwrap(),
        "miner7.near".parse().unwrap(),
        "miner8.near".parse().unwrap(),
        "miner9.near".parse().unwrap(),
        "miner10.near".parse().unwrap(),
    ];
    value
}

pub fn default_miners_commit_answer() -> Vec<String> {
    let value = vec![
        "422fa60e22dc75c98d21bb975323c5c0b854d6b0b7a63d6446b3bbb628b65a5b".to_string(),
        "c06a8aabd77066edbee09e50289c3cc1a3a57514bea9a9bcbb244559816ccf26".to_string(),
        "7fa05dacffc6bd12f708929057f259ab61505b6f21e45450d4c04509e0071e49".to_string(),
        "49284c05ff843c5a947bb041fafab9eb77685463f7c1e285274b878f2a2ee8a1".to_string(),
        "859597f6b7e5bc55a5ef630f6b1a7a8800740f8b77e6213fe314029010b132d4".to_string(),
        "51dde426921f48e3954ced820ec684bf480d66f0594ff5ffd85fd55e7a6b1736".to_string(),
        "24452398ffcafe810ec9c268d7637c9fafb1d407a76a7f219c176d4ae7d7e570".to_string(),
        "c062ac786582a16be008945533fe2db95de5d841dba864523bc3123c5642d346".to_string(),
        "21aeb50d9b89cfceccdf33741d037c66641e59acdf21f97457627d7f85db206e".to_string(),
        "47fb74320537a28d0130c7b2f00d4a75be7bdcf14b15930e36c336151de6dddc".to_string(),
    ];
    value
}

pub fn assert_log(event_name: &str, data: Vec<(&str, &str)>) {
    let logs = get_logs();
    assert_eq!(logs.len(), 1);

    let mut data_map = serde_json::Map::new();
    for (key, value) in data {
        data_map.insert(key.to_string(), json!(value));
    }

    let expected_event = json!({
        "standard": "emip001",
        "version": "1.0.0",
        "event": event_name,
        "data": [data_map]
    });

    // Deserialize both JSON strings into `Value` objects for comparison
    let log_event: Value = serde_json::from_str(logs[0].trim_start_matches("EVENT_JSON:")).unwrap();
    let expected_event: Value = expected_event;

    // Compare json objects
    assert_eq!(log_event, expected_event);
}

pub fn assert_logs(expected_logs: Vec<Log>) {
    let logs = get_logs();
    assert_eq!(logs.len(), expected_logs.len());

    for (i, expected_log) in expected_logs.iter().enumerate() {
        match expected_log {
            Log::Event { event_name, data } => {
                let mut data_map = serde_json::Map::new();
                for (key, value) in data {
                    data_map.insert(key.to_string(), value.clone());
                }

                let expected_event = json!({
                    "standard": "emip001",
                    "version": "1.0.0",
                    "event": event_name,
                    "data": [data_map]
                });

                // Deserialize both JSON strings into `Value` objects for comparison
                let log_event: Value = serde_json::from_str(logs[i].trim_start_matches("EVENT_JSON:")).unwrap();
                let expected_event: Value = expected_event;

                // Compare json objects
                assert_eq!(log_event, expected_event);
            }
            Log::Message(expected_text) => {
                assert_eq!(logs[i], *expected_text);
            }
        }
    }
}
