use near_sdk::AccountId;
use serde_json::json;

use common::constants::{
    COMMIT_VALIDATOR_TIME, DEFAULT_CULTURE, DEFAULT_DEPOSIT, DEFAULT_DEPOSIT_MINER, DEFAULT_DEPOSIT_PROTOCOL, DEFAULT_DEPOSIT_VALIDATOR,
    DEFAULT_MESSAGE_TO_REQUEST, DEFAULT_REQUEST_ID, DEFAULT_VALIDATOR_ANSWER, REVEAL_MINER_TIME, REVEAL_VALIDATOR_TIME, VALIDATOR_1, VALIDATOR_2,
};
use common::environment::Environment;
use common::types::Log;
use common::utils::{
    assert_logs, default_miners_commit_answer, generate_validator_answer, get_account_for_validator, get_default_protocol_account,
    get_default_validator_account, group_registered_miners,
};

use earthmind_rs::{Contract, Module, RevealMinerResult, RevealValidatorResult};

pub mod common;

#[test]
fn test_reveal_by_validator() {
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

    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();
    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    let registered_miners = group_registered_miners();
    let default_answer_miners = default_miners_commit_answer();

    for (index, miners) in registered_miners.clone().into_iter().enumerate() {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
        contract.register_miner();
        contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), default_answer_miners[index].clone());

        assert_logs(vec![
            Log::Event {
                event_name: "register_miner".to_string(),
                data: vec![("miner", json![miners])],
            },
            Log::Event {
                event_name: "commit_miner".to_string(),
                data: vec![
                    ("request_id", json![DEFAULT_REQUEST_ID]),
                    ("answer", json![default_answer_miners[index].clone()]),
                ],
            },
        ]);
    }

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();
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

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Success);
    assert_logs(vec![Log::Event {
        event_name: "reveal_validator".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);
}

#[test]
fn test_reveal_by_validator_when_miner_dont_have_a_commit_answer() {
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

    let registered_miners = group_registered_miners();

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
        contract.register_miner();
        assert_logs(vec![Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![miners])],
        }]);
    }

    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);
    assert_logs(vec![Log::Message("Account not registered a commit: miner1.near".to_string())]);
}

#[test]
fn test_reveal_by_validator_when_miner_have_a_commit_answer_but_not_revealed() {
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

    let validator = get_default_validator_account();
    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    let registered_miners = group_registered_miners();
    let default_answer_miners = default_miners_commit_answer();

    for (index, miners) in registered_miners.clone().into_iter().enumerate() {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
        contract.register_miner();
        contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), default_answer_miners[index].clone());

        assert_logs(vec![
            Log::Event {
                event_name: "register_miner".to_string(),
                data: vec![("miner", json![miners])],
            },
            Log::Event {
                event_name: "commit_miner".to_string(),
                data: vec![
                    ("request_id", json![DEFAULT_REQUEST_ID]),
                    ("answer", json![default_answer_miners[index].clone()]),
                ],
            },
        ]);
    }

    for miners in registered_miners {
        let miner_not_reveal: AccountId = "miner10.near".parse().unwrap();
        if miners == miner_not_reveal {
            break;
        }
        Environment::with_account(miners.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();
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

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);
    assert_logs(vec![Log::Message("Commit by miner not revealed: miner10.near".to_string())]);
}

#[test]
fn test_reveal_by_validator_when_validator_is_not_registered() {
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

    let validator = get_default_validator_account();
    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    Environment::with_account(validator).with_block_timestamp(COMMIT_VALIDATOR_TIME).create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    let unregistered_validator = get_account_for_validator(VALIDATOR_2);
    Environment::with_account(unregistered_validator)
        .with_block_timestamp(REVEAL_VALIDATOR_TIME)
        .create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![Log::Message("Validator is not registered: validator2.near".to_string())]);
}

#[test]
fn test_reveal_by_validator_when_request_is_not_registered() {
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

    let registered_miners = group_registered_miners();

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
        contract.register_miner();
        assert_logs(vec![Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![miners])],
        }]);
    }

    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();
    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let request_id_unregistered = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string();
    let answer: Vec<AccountId> = generate_validator_answer();
    let message = "It's a cool NFT".to_string();

    let result = contract.reveal_by_validator(request_id_unregistered, answer, message);

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![Log::Message(
        "Request is not registered: 0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae725".to_string(),
    )]);
}

#[test]
fn test_reveal_by_validator_when_proposal_is_already_reveal() {
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

    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    let registered_miners = group_registered_miners();
    let default_answer_miners = default_miners_commit_answer();

    for (index, miners) in registered_miners.clone().into_iter().enumerate() {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
        contract.register_miner();
        contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), default_answer_miners[index].clone());

        assert_logs(vec![
            Log::Event {
                event_name: "register_miner".to_string(),
                data: vec![("miner", json![miners])],
            },
            Log::Event {
                event_name: "commit_miner".to_string(),
                data: vec![
                    ("request_id", json![DEFAULT_REQUEST_ID]),
                    ("answer", json![default_answer_miners[index].clone()]),
                ],
            },
        ]);
    }

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();
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

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message.clone());

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![
        Log::Event {
            event_name: "reveal_validator".to_string(),
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
fn test_reveal_by_validator_when_answer_not_equal() {
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

    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    let registered_miners = group_registered_miners();
    let default_answer_miners = default_miners_commit_answer();

    for (index, miners) in registered_miners.clone().into_iter().enumerate() {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
        contract.register_miner();
        contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), default_answer_miners[index].clone());

        assert_logs(vec![
            Log::Event {
                event_name: "register_miner".to_string(),
                data: vec![("miner", json![miners])],
            },
            Log::Event {
                event_name: "commit_miner".to_string(),
                data: vec![
                    ("request_id", json![DEFAULT_REQUEST_ID]),
                    ("answer", json![default_answer_miners[index].clone()]),
                ],
            },
        ]);
    }

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();
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

    let extra_miner: AccountId = "miner11.near".parse().unwrap();
    let extra_miner_answer = "b574e5145b78602616f316e59a3556819d249c9297dfaab7938875bbb77c18d9".to_string();
    Environment::with_account(extra_miner.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
    contract.register_miner();

    contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), extra_miner_answer.clone());

    assert_logs(vec![
        Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![extra_miner])],
        },
        Log::Event {
            event_name: "commit_miner".to_string(),
            data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![extra_miner_answer])],
        },
    ]);
    Environment::with_account(extra_miner).with_block_timestamp(REVEAL_MINER_TIME).create();

    let answer = true;
    let message = "It's a cool NFT";
    contract.reveal_by_miner(DEFAULT_REQUEST_ID.to_string(), answer, message.to_string());

    assert_logs(vec![Log::Event {
        event_name: "reveal_miner".to_string(),
        data: vec![
            ("request_id", json![DEFAULT_REQUEST_ID]),
            ("answer", json![answer]),
            ("message", json![message]),
        ],
    }]);

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let mut answer: Vec<AccountId> = generate_validator_answer();
    answer[9] = "miner11.near".parse().unwrap();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message);

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![Log::Message("Answer don't match".to_string())]);
}

#[test]
fn test_reveal_by_validator_when_vote_for_miner_not_registered() {
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

    let registered_miners = group_registered_miners();

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();

        contract.register_miner();
        assert_logs(vec![Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![miners])],
        }]);
    }

    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();

    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let mut answer: Vec<AccountId> = generate_validator_answer();
    answer[9] = "miner12.near".parse().unwrap();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message);

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![Log::Message("Account not registered as miner: miner12.near".to_string())]);
}

#[test]
fn test_reveal_by_validator_when_miner_is_duplicated() {
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

    let registered_miners = group_registered_miners();

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT).create();
        contract.register_miner();
        assert_logs(vec![Log::Event {
            event_name: "register_miner".to_string(),
            data: vec![("miner", json![miners])],
        }]);
    }

    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();
    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    Environment::with_account(validator.clone())
        .with_block_timestamp(COMMIT_VALIDATOR_TIME)
        .create();

    contract.commit_by_validator(DEFAULT_REQUEST_ID.to_string(), DEFAULT_VALIDATOR_ANSWER.to_string());

    assert_logs(vec![Log::Event {
        event_name: "commit_validator".to_string(),
        data: vec![("request_id", json![DEFAULT_REQUEST_ID]), ("answer", json![DEFAULT_VALIDATOR_ANSWER])],
    }]);

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let mut answer: Vec<AccountId> = generate_validator_answer();
    answer[9] = "miner1.near".parse().unwrap();
    let message = "It's a cool NFT".to_string();
    let result = contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer.clone(), message);

    assert_eq!(result, RevealValidatorResult::Fail);

    assert_logs(vec![Log::Message("Repeated account: miner1.near".to_string())]);
}

#[test]
#[should_panic]
fn test_reveal_by_validator_when_dont_have_a_commit_answer() {
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

    let validator = get_default_validator_account();

    Environment::with_account(validator.clone())
        .with_attached_deposit(DEFAULT_DEPOSIT_VALIDATOR)
        .create();
    contract.register_validator();

    assert_logs(vec![Log::Event {
        event_name: "register_validator".to_string(),
        data: vec![("validator", json![VALIDATOR_1])],
    }]);

    let registered_miners = group_registered_miners();
    let default_answer_miners = default_miners_commit_answer();

    for (index, miners) in registered_miners.clone().into_iter().enumerate() {
        Environment::with_account(miners.clone()).with_attached_deposit(DEFAULT_DEPOSIT_MINER).create();
        contract.register_miner();
        contract.commit_by_miner(DEFAULT_REQUEST_ID.to_string(), default_answer_miners[index].clone());

        assert_logs(vec![
            Log::Event {
                event_name: "register_miner".to_string(),
                data: vec![("miner", json![miners])],
            },
            Log::Event {
                event_name: "commit_miner".to_string(),
                data: vec![
                    ("request_id", json![DEFAULT_REQUEST_ID]),
                    ("answer", json![default_answer_miners[index].clone()]),
                ],
            },
        ]);
    }

    for miners in registered_miners {
        Environment::with_account(miners.clone()).with_block_timestamp(REVEAL_MINER_TIME).create();
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

    Environment::with_account(validator).with_block_timestamp(REVEAL_VALIDATOR_TIME).create();

    let answer = generate_validator_answer();
    let message = "It's a cool NFT".to_string();
    contract.reveal_by_validator(DEFAULT_REQUEST_ID.to_string(), answer, message);
}
