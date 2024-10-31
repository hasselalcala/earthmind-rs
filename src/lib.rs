use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, PanicOnDefault};
use std::collections::HashSet;

pub use crate::constants::*;
pub use crate::events::*;
pub use crate::models::*;

mod constants;
mod events;
mod models;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    protocols: LookupMap<AccountId, Protocol>,
    requests: LookupMap<Hash, Request>,
    miners: LookupMap<AccountId, Stake>,
    validators: LookupMap<AccountId, Stake>,
}

#[near_bindgen]
impl Contract {
    #[allow(clippy::use_self)]
    #[init]
    pub fn new() -> Self {
        Self {
            protocols: LookupMap::new(b"protocols".to_vec()),
            requests: LookupMap::new(b"requests".to_vec()),
            miners: LookupMap::new(b"miners".to_vec()),
            validators: LookupMap::new(b"validators".to_vec()),
        }
    }

    pub fn register_protocol(&mut self, culture: String, modules: Vec<Module>) -> RegisterProtocolResult {
        let new_account = env::predecessor_account_id();
        let registration_fee = env::attached_deposit();

        if registration_fee < PROTOCOL_REGISTRATION_FEE {
            panic!("Deposit is less than the required to register");
        }

        if self.is_protocol_registered(new_account.clone()) {
            log!("Attempted to register an already registered account: {}", new_account);
            return RegisterProtocolResult::AlreadyRegistered;
        }

        let new_protocol = Protocol {
            account: new_account.clone(),
            culture,
            modules,
            registration_fee,
        };

        self.protocols.insert(new_account.clone(), new_protocol);

        let register_protocol_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterProtocol(vec![RegisterProtocolLog { account: new_account }]),
        };

        log!(&register_protocol_log.to_string());

        RegisterProtocolResult::Success
    }

    pub fn is_protocol_registered(&self, account: AccountId) -> bool {
        self.protocols.contains_key(&account)
    }

    pub fn register_miner(&mut self) -> RegisterMinerResult {
        let new_miner_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        if deposit < MIN_MINER_STAKE {
            panic!("Miner deposit is less than the minimum stake");
        }

        // @dev Validate the miner is not already registered
        if self.is_miner_registered(new_miner_id.clone()) {
            log!("Attempted to register an already registered miner: {}", new_miner_id);
            return RegisterMinerResult::AlreadyRegistered;
        }

        self.miners.insert(new_miner_id.clone(), deposit);

        let register_miner_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterMiner(vec![RegisterMinerLog { miner: new_miner_id }]),
        };

        log!(&register_miner_log.to_string());

        RegisterMinerResult::Success
    }

    pub fn is_miner_registered(&self, miner_id: AccountId) -> bool {
        self.miners.contains_key(&miner_id)
    }

    pub fn register_validator(&mut self) -> RegisterValidatorResult {
        let new_validator_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        if deposit < MIN_VALIDATOR_STAKE {
            panic!("Validator deposit is less than the minimum stake");
        }

        if self.is_validator_registered(new_validator_id.clone()) {
            log!("Attempted to register an already registered validator: {}", new_validator_id);
            return RegisterValidatorResult::AlreadyRegistered;
        }

        self.validators.insert(new_validator_id.clone(), deposit);

        let register_validator_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterValidator(vec![RegisterValidatorLog { validator: new_validator_id }]),
        };

        log!(&register_validator_log.to_string());

        RegisterValidatorResult::Success
    }

    pub fn is_validator_registered(&self, validator_id: AccountId) -> bool {
        self.validators.contains_key(&validator_id)
    }

    pub fn request_governance_decision(&mut self, message: String) -> RegisterRequestResult {
        let sender_account = env::predecessor_account_id();

        let concatenated_answer = format!("{}{}", sender_account, message);
        let new_request_id = env::keccak256(concatenated_answer.as_bytes());
        let new_request_id_hex = hex::encode(new_request_id);

        //@dev verify that user is registerd in the protocol
        if !self.is_protocol_registered(sender_account.clone()) {
            panic!("Account unregistered: {}", sender_account);
        }

        //@dev Validate the request is not already registered
        if self.get_request_by_id(new_request_id_hex.clone()) {
            log!("Attempted to register an already registered request: {}", new_request_id_hex);
            return RegisterRequestResult::AlreadyRegistered;
        }

        let new_request = Request {
            sender: sender_account,
            request_id: new_request_id_hex.clone(),
            start_time: env::block_timestamp(),
            miners_proposals: LookupMap::new(b"miner_proposal".to_vec()),
            validators_proposals: LookupMap::new(b"validator_proposal".to_vec()),
            votes_for_miners: LookupMap::new(b"votes_miners".to_vec()),
            miner_keys: Vec::new(),
            top_ten: Vec::new(),
        };

        // @dev We store the key of the request as the hash of the message
        self.requests.insert(new_request_id_hex.clone(), new_request);

        let register_request_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterRequest(vec![RegisterRequestLog {
                request_id: new_request_id_hex,
            }]),
        };

        log!(&register_request_log.to_string());

        RegisterRequestResult::Success
    }

    pub fn get_request_by_id(&self, request_id: Hash) -> bool {
        self.requests.contains_key(&request_id)
    }

    fn get_request_by_id_mut(&mut self, request_id: Hash) -> Option<&mut Request> {
        self.requests.get_mut(&request_id)
    }

    fn get_stage(start_time: u64) -> RequestState {
        let elapsed = env::block_timestamp() - start_time;

        if start_time == 0 {
            RequestState::NonStarted
        } else if elapsed < COMMIT_MINER_DURATION {
            RequestState::CommitMiners
        } else if elapsed < COMMIT_MINER_DURATION + REVEAL_MINER_DURATION {
            RequestState::RevealMiners
        } else if elapsed < COMMIT_MINER_DURATION + REVEAL_MINER_DURATION + COMMIT_VALIDATOR_DURATION {
            RequestState::CommitValidators
        } else if elapsed < COMMIT_MINER_DURATION + REVEAL_MINER_DURATION + COMMIT_VALIDATOR_DURATION + REVEAL_VALIDATOR_DURATION {
            RequestState::RevealValidators
        } else {
            RequestState::Ended
        }
    }

    pub fn hash_miner_answer(self, request_id: Hash, answer: bool, message: String) -> Hash {
        let miner = env::predecessor_account_id();

        let concatenated_answer = format!("{}{}{}{}", request_id, miner, answer, message);
        let value = env::keccak256(concatenated_answer.as_bytes());

        //@dev Return the hash of the answer
        hex::encode(value)
    }

    pub fn commit_by_miner(&mut self, request_id: Hash, answer: Hash) -> CommitMinerResult {
        let miner = env::predecessor_account_id();

        if !self.is_miner_registered(miner.clone()) {
            log!("Miner not registered: {}", miner);
            return CommitMinerResult::Fail;
        }

        match self.get_request_by_id_mut(request_id.clone()) {
            Some(request) => {
                assert_eq!(Self::get_stage(request.start_time), RequestState::CommitMiners, "Not at CommitMiners stage");

                if request.miners_proposals.get(&miner).is_some() {
                    log!("This miner have a commit answer: {}", miner);
                    return CommitMinerResult::Fail;
                }

                let proposal = MinerProposal {
                    proposal_hash: answer.clone(),
                    answer: false,
                    is_revealed: false,
                };

                // @dev Insert miners_proposals using a mut reference
                request.miners_proposals.insert(miner, proposal);

                let commit_miner_log = EventLog {
                    standard: "emip001".to_string(),
                    version: "1.0.0".to_string(),
                    event: EventLogVariant::CommitMiner(vec![CommitMinerLog { request_id, answer }]),
                };

                log!(&commit_miner_log.to_string());

                CommitMinerResult::Success
            }
            None => {
                log!("Request is not registered: {}", request_id);
                CommitMinerResult::Fail
            }
        }
    }

    pub fn hash_validator_answer(self, request_id: String, answer: Vec<AccountId>, message: String) -> Hash {
        let validator = env::predecessor_account_id();

        require!(answer.len() == 10, "Invalid answer");

        let mut concatenated_answer: Vec<u8> = Vec::new();

        concatenated_answer.extend_from_slice(request_id.as_bytes());
        concatenated_answer.extend_from_slice(validator.as_bytes());

        let value: Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(&value);
        concatenated_answer.extend_from_slice(message.as_bytes());

        let value = env::keccak256(&concatenated_answer);

        //@dev Return the hash of the answer
        hex::encode(value)
    }

    pub fn commit_by_validator(&mut self, request_id: String, answer: Hash) -> CommitValidatorResult {
        let validator = env::predecessor_account_id();

        if !self.is_validator_registered(validator.clone()) {
            log!("Validator is not registered: {}", validator);
            return CommitValidatorResult::Fail;
        }

        match self.get_request_by_id_mut(request_id.clone()) {
            Some(request) => {
                assert_eq!(
                    Self::get_stage(request.start_time),
                    RequestState::CommitValidators,
                    "Not at CommitValidator stage"
                );

                if request.validators_proposals.get(&validator).is_some() {
                    log!("This validator have a commit answer: {}", validator);
                    return CommitValidatorResult::Fail;
                }

                let proposal = ValidatorProposal {
                    proposal_hash: answer.clone(),
                    is_revealed: false,
                    miner_addresses: Vec::new(),
                };

                // @dev Insert miners_proposals using a mut reference
                request.validators_proposals.insert(validator, proposal);

                let commit_validator_log = EventLog {
                    standard: "emip001".to_string(),
                    version: "1.0.0".to_string(),
                    event: EventLogVariant::CommitValidator(vec![CommitValidatorLog { request_id, answer }]),
                };

                log!(&commit_validator_log.to_string());

                CommitValidatorResult::Success
            }
            None => {
                log!("Request is not registered: {}", request_id);
                CommitValidatorResult::Fail
            }
        }
    }

    pub fn reveal_by_miner(&mut self, request_id: String, answer: bool, message: String) -> RevealMinerResult {
        let miner = env::predecessor_account_id();

        if !self.is_miner_registered(miner.clone()) {
            log!("Miner not registered: {}", miner);
            return RevealMinerResult::Fail;
        }

        if self.get_request_by_id_mut(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealMinerResult::Fail;
        }

        let complete_request = self
            .get_request_by_id_mut(request_id.clone())
            .map_or_else(|| panic!("Request not found"), |request| request);

        assert_eq!(
            Self::get_stage(complete_request.start_time),
            RequestState::RevealMiners,
            "Not at RevealMiners stage"
        );

        let save_proposal = complete_request
            .miners_proposals
            .get_mut(&miner)
            .map_or_else(|| panic!("proposal not found"), |proposal| proposal);

        if save_proposal.is_revealed {
            log!("Proposal already revealed");
            return RevealMinerResult::Fail;
        }

        let concatenated_answer = format!("{}{}{}{}", request_id, miner, answer, message);
        let hash_value = env::keccak256(concatenated_answer.as_bytes());
        let answer_to_verify = hex::encode(hash_value);

        if save_proposal.proposal_hash != answer_to_verify {
            log!("Answer don't match");
            return RevealMinerResult::Fail;
        }

        save_proposal.answer = answer;
        save_proposal.is_revealed = true;

        let reveal_miner_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RevealMiner(vec![RevealMinerLog { request_id, answer, message }]),
        };

        env::log_str(&reveal_miner_log.to_string());

        RevealMinerResult::Success
    }

    pub fn reveal_by_validator(&mut self, request_id: String, answer: Vec<AccountId>, message: String) -> RevealValidatorResult {
        let validator = env::predecessor_account_id();

        if !self.is_validator_registered(validator.clone()) {
            log!("Validator is not registered: {}", validator);
            return RevealValidatorResult::Fail;
        }

        //@dev verify that the answer vector have 10 elements
        if answer.len() != 10 {
            log!("Invalid answer");
            return RevealValidatorResult::Fail;
        }

        //@dev verify that the answer don't have repeated account
        let mut set = HashSet::new();
        for accounts in answer.clone() {
            if !set.insert(accounts.clone()) {
                log!("Repeated account: {}", accounts);
                return RevealValidatorResult::Fail;
            }
        }

        //@dev verify that every account is registered as miner
        for accounts in answer.clone() {
            if !self.miners.contains_key(&accounts) {
                log!("Account not registered as miner: {}", accounts);
                return RevealValidatorResult::Fail;
            }
        }

        if self.get_request_by_id_mut(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealValidatorResult::Fail;
        }

        let complete_request = self
            .get_request_by_id_mut(request_id.clone())
            .map_or_else(|| panic!("Request not found"), |request| request);

        assert_eq!(
            Self::get_stage(complete_request.start_time),
            RequestState::RevealValidators,
            "Not at RevealValidators stage"
        );

        let save_proposal = complete_request
            .validators_proposals
            .get_mut(&validator)
            .map_or_else(|| panic!("proposal not found"), |proposal| proposal);

        if save_proposal.is_revealed {
            log!("Proposal already revealed");
            return RevealValidatorResult::Fail;
        }

        // @dev verify that miners in the answer have a commit answer
        for accounts in answer.clone() {
            if !complete_request.miners_proposals.contains_key(&accounts) {
                log!("Account not registered a commit: {}", accounts);
                return RevealValidatorResult::Fail;
            }
        }

        //@dev verify that the commit answer by miner was revealed
        for accounts in answer.clone() {
            let miner_proposal = complete_request
                .miners_proposals
                .get(&accounts)
                .map_or_else(|| panic!("proposal not found"), |proposal| proposal);

            if !miner_proposal.is_revealed {
                log!("Commit by miner not revealed: {}", accounts);
                return RevealValidatorResult::Fail;
            }
        }

        let mut concatenated_answer: Vec<u8> = Vec::new();

        concatenated_answer.extend_from_slice(request_id.as_bytes());
        concatenated_answer.extend_from_slice(validator.as_bytes());

        let value: Vec<u8> = answer.iter().flat_map(|id| id.as_bytes()).copied().collect();
        concatenated_answer.extend_from_slice(&value);
        concatenated_answer.extend_from_slice(message.as_bytes());

        let value = env::keccak256(&concatenated_answer);
        let hash_answer = hex::encode(value);

        if save_proposal.proposal_hash != hash_answer {
            log!("Answer don't match");
            //log!("save answer: {}", save_proposal.proposal_hash);
            //log!("hash_answer calculated: {}", hash_answer);
            return RevealValidatorResult::Fail;
        }

        save_proposal.is_revealed = true;
        let answer_for_log = answer.clone();

        for addresses in answer {
            save_proposal.miner_addresses.push(addresses.clone());

            //@dev Find the miner votes and add 1
            if complete_request.votes_for_miners.contains_key(&addresses) {
                match complete_request.votes_for_miners.get(&addresses) {
                    Some(num_votes) => complete_request.votes_for_miners.insert(addresses, *num_votes + 1),
                    None => panic!("miner not found"),
                };
            } else {
                complete_request.votes_for_miners.insert(addresses.clone(), 1);
                complete_request.miner_keys.push(addresses);
            }
        }

        let reveal_validator_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RevealValidator(vec![RevealValidatorLog {
                request_id,
                answer: answer_for_log,
                message,
            }]),
        };

        env::log_str(&reveal_validator_log.to_string());
        RevealValidatorResult::Success
    }

    pub fn votes_for_miner(&mut self, request_id: String, miner_id: AccountId) {
        if self.get_request_by_id_mut(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
        }

        let complete_request = self
            .get_request_by_id_mut(request_id)
            .map_or_else(|| panic!("Request not found"), |request| request);

        match complete_request.votes_for_miners.get(&miner_id) {
            Some(votes) => log!("{} have {} votes", miner_id, *votes),
            None => log!("miner don't have votes"),
        };
    }

    pub fn get_top_10_voters(&mut self, request_id: String) -> Vec<(AccountId, i32)> {
        if self.get_request_by_id_mut(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
        }

        let complete_request = self
            .get_request_by_id_mut(request_id.clone())
            .map_or_else(|| panic!("Request not found"), |request| request);

        assert_eq!(Self::get_stage(complete_request.start_time), RequestState::Ended, "Not stage ended");

        let mut vote_result = Vec::new();

        for miner_keys in complete_request.miner_keys.iter() {
            if let Some(votes) = complete_request.votes_for_miners.get(miner_keys) {
                vote_result.push((miner_keys.clone(), *votes));
            }
        }

        vote_result.sort_by(|a, b| b.1.cmp(&a.1));

        let top_ten: Vec<_> = vote_result.iter().take(10).cloned().collect();
        complete_request.top_ten.clone_from(&top_ten);

        let top_ten_log = EventLog {
            standard: "emip001".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::ToptenMiners(vec![ToptenMinersLog {
                request_id,
                topten: top_ten.clone(),
            }]),
        };
        env::log_str(&top_ten_log.to_string());
        top_ten
    }
}

// Test private function "get_request_by_id_mut"

#[cfg(test)]
mod test {
    use super::*;
    use near_sdk::{
        env,
        test_utils::{get_logs, VMContextBuilder},
        testing_env, AccountId, NearToken,
    };

    fn get_context(predecessor_account_id: AccountId, block_timestamp: u64, attached_deposit: NearToken) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(predecessor_account_id)
            .block_timestamp(block_timestamp)
            .attached_deposit(attached_deposit);
        builder
    }

    #[test]
    fn test_request_governance_decision() {
        let mut contract = Contract::new();

        let context = get_context("account1.near".parse().unwrap(), 100000000, NearToken::from_near(5));
        testing_env!(context.build());

        let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
        contract.register_protocol("Governance decision".to_string(), modules);

        let message = "Should we add this new NFT to our protocol?";
        let result_1 = contract.request_governance_decision(message.to_string());
        assert_eq!(result_1, RegisterRequestResult::Success);

        let sender_account = env::predecessor_account_id();
        let concatenated_answer = format!("{}{}", sender_account, message);
        let request_id = env::keccak256(concatenated_answer.as_bytes());
        let request_id_hex = hex::encode(request_id);

        assert!(contract.get_request_by_id_mut(request_id_hex).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(
            logs[0],
            r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_protocol","data":[{"account":"account1.near"}]}"#
        );

        assert_eq!(
            logs[1],
            r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"73ead60176d724e462dbfa8d49506177bb13bec748cf5af5019b6d1da63e204b"}]}"#
        );
    }

    #[test]
    fn test_multiple_request_governance_decision() {
        let mut contract = Contract::new();

        let context = get_context("account1.near".parse().unwrap(), 100000000, NearToken::from_near(5));
        testing_env!(context.build());

        let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
        contract.register_protocol("Governance decision".to_string(), modules);

        let message = "Should we add this new NFT to our protocol?";
        let result_1 = contract.request_governance_decision(message.to_string());
        assert_eq!(result_1, RegisterRequestResult::Success);

        let sender_account = env::predecessor_account_id();
        let concatenated_answer = format!("{}{}", sender_account, message);
        let request_id = env::keccak256(concatenated_answer.as_bytes());
        let request_id_hex = hex::encode(request_id);

        assert!(contract.get_request_by_id_mut(request_id_hex).is_some());

        let logs = get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(
            logs[0],
            r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_protocol","data":[{"account":"account1.near"}]}"#
        );

        assert_eq!(
            logs[1],
            r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"73ead60176d724e462dbfa8d49506177bb13bec748cf5af5019b6d1da63e204b"}]}"#
        );

        let context = get_context("account2.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(25)));
        testing_env!(context.build());

        let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
        contract.register_protocol("Governance decision for ethereum".to_string(), modules);

        let message_2 = "Should we add this to our protocol?";
        let result_2 = contract.request_governance_decision(message_2.to_string());
        assert_eq!(result_2, RegisterRequestResult::Success);

        let sender_account_2 = env::predecessor_account_id();
        let concatenated_answer_2 = format!("{}{}", sender_account_2, message_2);
        let request_id_2 = env::keccak256(concatenated_answer_2.as_bytes());
        let request_id_hex_2 = hex::encode(request_id_2);

        assert!(contract.get_request_by_id_mut(request_id_hex_2).is_some());
        let logs = get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(
            logs[0],
            r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_protocol","data":[{"account":"account2.near"}]}"#
        );
        assert_eq!(
            logs[1],
            r#"EVENT_JSON:{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{"request_id":"c4b35bc95d323446f6f800e7639457cddc34c7f768772e4871adf2dd34f89ed8"}]}"#
        );
    }

    #[test]
    #[should_panic(expected = "Account unregistered: account1.near")]
    fn test_request_governance_decision_with_an_unregistered_protocol() {
        let mut contract = Contract::new();

        let context = get_context("account1.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(2)));
        testing_env!(context.build());

        let message = "Should we add this new NFT to our protocol?";

        contract.request_governance_decision(message.to_string());
    }

    #[test]
    fn test_get_request_by_id_mut() {
        let mut contract = Contract::new();

        let context = get_context("account1.near".parse().unwrap(), 100000000, NearToken::from_near(5));
        testing_env!(context.build());

        let modules = vec![Module::TextPrompting, Module::ObjectRecognition];
        contract.register_protocol("Governance decision for ethereum".to_string(), modules);

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "73ead60176d724e462dbfa8d49506177bb13bec748cf5af5019b6d1da63e204b";
        assert!(contract.get_request_by_id_mut(request_id.to_string()).is_some());
    }

    #[test]
    #[should_panic(expected = "Account unregistered: account1.near")]
    fn test_get_request_by_id_mut_an_unregistered_account() {
        let context = get_context("account1.near".parse().unwrap(), 100000000, NearToken::from_yoctonear(10u128.pow(24)));
        testing_env!(context.build());

        let mut contract = Contract::new();

        let message = "Should we add this new NFT to our protocol?";
        contract.request_governance_decision(message.to_string());

        let request_id = "73ead60176d724e462dbfa8d49506177bb13bec748cf5af5019b6d1da63e204b";
        assert!(contract.get_request_by_id_mut(request_id.to_string()).is_some());
    }

    #[test]
    fn test_get_request_by_id_mut_when_not_registered() {
        let mut contract = Contract::new();
        let request_id = "0504fbdd23f833749a13dcde971238ba62bdde0ed02ea5424f5a522f50fae727";

        assert!(contract.get_request_by_id_mut(request_id.to_string()).is_none());
    }
}
