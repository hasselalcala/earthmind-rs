# EarthMind Improvement Proposal 001 (Standard Events)

This document describes the Earthmind standard events.

## Specification

Notes:
- This standard uses JSON for serialization of results.
- The contract must track the change in storage when adding to and removing from collections. 

### Contract interface

```
pub struct MinerProposal {
    pub proposal_hash: Hash,
    pub answer: bool,
    pub is_revealed: bool,
}

pub struct ValidatorProposal {
    pub proposal_hash: Hash,
    pub is_revealed: bool,
    pub miner_addresses: Vec<AccountId>,
}

pub struct Request {
    pub sender: AccountId,
    pub request_id: String,
    pub start_time: u64,
    pub miners_proposals: LookupMap<AccountId, MinerProposal>,
    pub validators_proposals: LookupMap<AccountId, ValidatorProposal>,
}

/********************/
/* REGISTER METHODS */
/********************/

// Register miner.

// Requirements. 
// * A new miner must attach a deposit of 1 near.
// * Contract must panic if:
// - the deposit is less than 1 near
// - the miner is already registered

pub fn register_miner(&mut self) -> RegisterMinerResult {}

Returns "Success" if the miner was registered.

// Register validator.

// Requirements. 
// * A new validator must attach a deposit of 10 near.
// * Contract must panic if:
// - the deposit is less than 10 near
// - the validator is already registered

pub fn register_validator(&mut self) -> RegisterValidatorResult {}

Returns "Success" if the validator was registered.

// Register request.

// Requirements. 
// * Contract must panic if request already exists

// Arguments.
// * `message`: we send the question
pub fn request_governance_decision(&mut self, message: String) -> RegisterRequestResult {}
    
Returns "Success" if the request was registered.

/******************/
/* COMMIT METHODS */
/******************/

// Commit by miner.

//Requirements.
// * Verify that miner is already registered. 
// * Verify that request already exist.
// * Verify that is time to commit.
// * Verify that miner is not trying to commit a second proposal. 

// Arguments.
// * request_id: expected request ID. A value that was genererated when a new request was send.
// * answer: expected the hash answer to commit.

pub fn commit_by_miner(&mut self, request_id: Hash, answer: Hash) -> CommitMinerResult {}

Return "Success" if the commit was registered.

// Commit by validator

// Requirements.
// * Verify that validator is already registered. 
// * Verify that request already exist.
// * Verify that is time to commit.
// * Verify that validator is not trying to commit a second proposal. 

// Arguments.
// * request_id: expected request ID. A value that was genererated when a new request was send.
// * answer: expected the hash answer to commit.

pub fn commit_by_validator(&mut self, request_id: Hash, answer: Hash) -> CommitMinerResult {}

Return "Success" if the commit was registered.

/******************/
/* REVEAL METHODS */
/******************/

//Reveal by miner

// Requirements. 
// * Verify that miner is already registered. 
// * Verify that request already exist.
// * Verify that is time to reveal.
// * Verify that miner have a commit answer.
// * Verify that miner is not trying to reveal a proposal that was already revealed. 
// * Verify that the hash generated using answer and message arguments are equal to the hash that was commited.

//Arguments
// * request_id: expected request ID. A value that was genererated when a new request was send.
// * answer: expected a bool value that was used to generate the hashed answer that was commited
// * message: expected a message that was used to generate the hashed answer that was commited

pub fn reveal_by_validator(&mut self, request_id: String, answer: Vec<AccountId>, message: String) -> RevealValidatorResult {}

Return "Success" if the proposal was revealed.
    
//Reveal by validator

// Requirements. 
// * Verify that validator is already registered. 
// * Verify that request already exist.
// * Verify that is time to reveal.
// * Verify that validator have a commit answer.
// * Verify that validator is not trying to reveal a proposal that was already revealed. 
// * Verify that the hash generated using answer and message arguments are equal to the hash that was commited.

//Arguments
// * request_id: expected request ID. A value that was genererated when a new request was send.
// * answer: expected an AccountID vector which was used to generate the hashed answer that was commited
// * message: expected a message that was used to generate the hashed answer that was commited

pub fn reveal_by_miner(&mut self, request_id: String, answer: bool, message: String) -> RevealMinerResult {}

Return "Success" if the proposal was revealed.
```

### Events

#### Events interface

```
pub struct EventLog {
    pub standard: "emip001",
    pub version: "1.0.0",
    pub event: "RegisterMiner" | "RegisterValidator" | "RegisterRequest" | "CommitMiner" | "CommitValidator" | "RevealMiner" | "RevealValidator",
    data: RegisterMinerLog[] | RegisterValidatorLog[] | RegisterRequestLog[] | CommitMinerLog[] | CommitValidatorLog[] | RevealMinerLog[] | RevealValidatorLog[],
}
```

```
// An event log to capture register miners
// Arguments
// * miner: "hassel.near"
pub struct RegisterMinerLog {
    pub miner: AccountId,
}

// An event log to capture register validators
// Arguments
// * validator: "edson.near"
pub struct RegisterValidatorLog {
    pub validator: AccountId,
}

// An event log to capture register requests
// Arguments
// * request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
pub struct RegisterRequestLog {
    pub request_id: String,
}

// An event log to capture register commit by miner
// Arguments
// * request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
// * answer: "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"
pub struct CommitMinerLog {
    pub request_id: String,
    pub answer: Hash,
}

// An event log to capture register commit by validator
// Arguments
// * request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
// * answer: "3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"
pub struct CommitValidatorLog {
    pub request_id: String,
    pub answer: Hash,
}

// An event log to capture reveal by miner
// Arguments
// * request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
// * answer: true,
// * message: "It's a cool NFT"
pub struct RevealMinerLog {
    pub request_id: String,
    pub answer: bool,
    pub message: String,
}

// An event log to capture reveal by validator
// Arguments
// * request_id: "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"
// * answer: [ "hassel.near", "edson.near", "anne.near", "bob.near", "alice.near", "john.near", "harry.near", "scott.near", "felix.near", "margaret.near"],
// * message: "It's a cool NFT"
pub struct RevealValidatorLog {
    pub request_id: String,
    pub answer: Vec<AccountId>,
    pub message: String,
}
```

### Examples

Register miner:

```
EVENT_JSON:{
    "standard":"emip001",
    "version":"1.0.0",
    "event":"register_miner",
    "data":[{"miner":"hassel.near"},{"miner":"edson.near"}]
}
```

Register validator:

```
EVENT_JSON:{
    "standard":"emip001",
    "version":"1.0.0",
    "event":"register_validator",
    "data":[{"validator":"hassel.near"},{"validator":"edson.near"}]
}
```

Register request:

```
EVENT_JSON:{
    "standard":"emip001",
    "version":"1.0.0",
    "event":"register_request",
    "data": [{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726"},{"request_id":"38d15af71379737839e4738066fd4091428081d6a57498b2852337a195bc9f5f"}]
}
```

Commit miner:

```
EVENT_JSON:{
    "standard":"emip001",
    "version":"1.0.0",
    "event":"commit_miner",
    "data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]
}
```

Commit validator:

```
EVENT_JSON:{
    "standard":"emip001",
    "version":"1.0.0",
    "event":"commit_validator",
    "data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":"3910deb8f11de66388bddcc1eb1bf1e33319b71a18df2c1019e6d72c6d00f464"}]
}
```

Reveal miner:

```
EVENT_JSON:{
    "standard":"emip001",
    "version":"1.0.0",
    "event":"reveal_miner",
    "data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":true,"message":"It's a cool NFT"}]
}

```


Reveal validator:

```
EVENT_JSON:{
    "standard":"emip001",
    "version":"1.0.0",
    "event":"reveal_validator",
    "data":[{"request_id":"0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae726","answer":["hassel.near","edson.near","anne.near","bob.near","alice.near","john.near","harry.near","scott.near","felix.near","margaret.near"],"message":"It's a cool NFT"}]
}
```
