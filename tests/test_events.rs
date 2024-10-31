use earthmind_rs::{
    CommitMinerLog, CommitValidatorLog, EventLog, EventLogVariant, RegisterMinerLog, RegisterProtocolLog, RegisterRequestLog, RegisterValidatorLog,
    RevealMinerLog, RevealValidatorLog, ToptenMinersLog,
};

#[test]
fn test_format_register_protocol() {
    let expected =
        r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_protocol","data":[{"account":"miner1.near"},{"account":"validator1.near"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RegisterProtocol(vec![
            RegisterProtocolLog {
                account: "miner1.near".parse().unwrap(),
            },
            RegisterProtocolLog {
                account: "validator1.near".parse().unwrap(),
            },
        ]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_register_miner() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{"miner":"miner1.near"},{"miner":"miner2.near"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RegisterMiner(vec![
            RegisterMinerLog {
                miner: "miner1.near".parse().unwrap(),
            },
            RegisterMinerLog {
                miner: "miner2.near".parse().unwrap(),
            },
        ]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_register_validator() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_validator","data":[{"validator":"validator1.near"},{"validator":"validator2.near"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RegisterValidator(vec![
            RegisterValidatorLog {
                validator: "validator1.near".parse().unwrap(),
            },
            RegisterValidatorLog {
                validator: "validator2.near".parse().unwrap(),
            },
        ]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_register_request() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"},{"request_id":"38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RegisterRequest(vec![
            RegisterRequestLog {
                request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            },
            RegisterRequestLog {
                request_id: "38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f".to_string(),
            },
        ]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_commit_miner() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::CommitMiner(vec![CommitMinerLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            answer: "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string(),
        }]),
    };

    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_commit_validator() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"commit_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::CommitValidator(vec![CommitValidatorLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            answer: "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464".to_string(),
        }]),
    };

    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_reveal_miner() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_miner","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":true,"message":"It's a cool NFT"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RevealMiner(vec![RevealMinerLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            answer: true,
            message: "It's a cool NFT".to_string(),
        }]),
    };

    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_reveal_validator() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"reveal_validator","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":["miner1.near","miner2.near","miner3.near","miner4.near","miner5.near","miner6.near","miner7.near","miner8.near","miner9.near","miner10.near"],"message":"It's a cool NFT"}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::RevealValidator(vec![RevealValidatorLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            answer: vec![
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
            ],
            message: "It's a cool NFT".to_string(),
        }]),
    };

    assert_eq!(expected, log.to_string());
}

#[test]
fn test_format_topten_miners() {
    let expected = r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"topten_miners","data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","topten":[["miner1.near",3],["miner2.near",3],["miner3.near",3],["miner4.near",3],["miner5.near",3],["miner6.near",3],["miner7.near",3],["miner8.near",3],["miner9.near",3],["miner10.near",3]]}]}"#;
    let log = EventLog {
        standard: "emip001".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::ToptenMiners(vec![ToptenMinersLog {
            request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726".to_string(),
            topten: vec![
                ("miner1.near".parse().unwrap(), 3),
                ("miner2.near".parse().unwrap(), 3),
                ("miner3.near".parse().unwrap(), 3),
                ("miner4.near".parse().unwrap(), 3),
                ("miner5.near".parse().unwrap(), 3),
                ("miner6.near".parse().unwrap(), 3),
                ("miner7.near".parse().unwrap(), 3),
                ("miner8.near".parse().unwrap(), 3),
                ("miner9.near".parse().unwrap(), 3),
                ("miner10.near".parse().unwrap(), 3),
            ],
        }]),
    };
    assert_eq!(expected, log.to_string());
}
