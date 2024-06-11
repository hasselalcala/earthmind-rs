use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::{LookupMap, Vector};
use near_sdk::{env, log, near_bindgen, require, AccountId, PanicOnDefault};

pub use crate::events::*;
pub use crate::models::*;

mod events;
mod models;

type Hash = String;
const TWO_MINUTES: u64 = 2 * 60 * 1_000_000_000; // 2 minutes in nanoseconds
const COMMIT_MINER_DURATION: u64 = TWO_MINUTES;
const REVEAL_MINER_DURATION: u64 = TWO_MINUTES;
const COMMIT_VALIDATOR_DURATION: u64 = TWO_MINUTES;
const REVEAL_VALIDATOR_DURATION: u64 = TWO_MINUTES;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    requests: Vector<Request>,
    miners: Vector<AccountId>,
    validators: Vector<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[allow(clippy::use_self)]
    #[init]
    pub fn new() -> Self {
        Self {
            requests: Vector::new(b"r"),
            miners: Vector::new(b"m"),
            validators: Vector::new(b"v"),
        }
    }

    pub fn register_miner(&mut self) -> RegisterMinerResult {
        let new_miner_id = env::predecessor_account_id();

        // @dev Validate the miner is not already registered
        if self.get_register_miner(new_miner_id.clone()).is_some() {
            log!(
                "Attempted to register an already registered miner: {}",
                new_miner_id
            );
            return RegisterMinerResult::AlreadyRegistered;
        }

        self.miners.push(new_miner_id.clone());

        let register_miner_log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterMiner(vec![RegisterMinerLog {
                miner: new_miner_id,
            }]),
        };
        env::log_str(&register_miner_log.to_string());
        RegisterMinerResult::Success
    }

    pub fn get_register_miner(&self, miner_id: AccountId) -> Option<&AccountId> {
        self.miners.iter().find(|&miner| *miner == miner_id)
    }

    pub fn register_validator(&mut self) -> RegisterValidatorResult {
        let new_validator_id = env::predecessor_account_id();

        if self
            .get_register_validator(new_validator_id.clone())
            .is_some()
        {
            log!(
                "Attempted to register an already registered validator: {}",
                new_validator_id
            );
            return RegisterValidatorResult::AlreadyRegistered;
        }

        self.validators.push(new_validator_id.clone());

        let register_validator_log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterValidator(vec![RegisterValidatorLog {
                validator: new_validator_id,
            }]),
        };
        env::log_str(&register_validator_log.to_string());

        RegisterValidatorResult::Success
    }

    pub fn get_register_validator(&self, validator_id: AccountId) -> Option<&AccountId> {
        self.validators
            .iter()
            .find(|&validator| *validator == validator_id)
    }

    pub fn request_governance_decision(&mut self, message: String) -> RegisterRequestResult {
        let new_request_id = env::keccak256(message.as_bytes());
        let new_request_id_hex = hex::encode(new_request_id);

        //@dev Validate the request is not already registered
        if self.get_request_by_id(new_request_id_hex.clone()).is_some() {
            log!(
                "Attempted to register an already registered request: {}",
                new_request_id_hex
            );
            return RegisterRequestResult::AlreadyRegistered;
        }

        let new_request = Request {
            sender: env::predecessor_account_id(),
            request_id: new_request_id_hex.clone(),
            start_time: env::block_timestamp(),
            miners_proposals: LookupMap::new(b"m"),
            validators_proposals: LookupMap::new(b"v"),
            validators_votes_to_miner: LookupMap::new(b"v"),
        };

        self.requests.push(new_request);

        let register_request_log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RegisterRequest(vec![RegisterRequestLog {
                request_id: new_request_id_hex,
            }]),
        };
        env::log_str(&register_request_log.to_string());
        RegisterRequestResult::Success
    }

    pub fn get_request_by_id(&mut self, request_id: String) -> Option<&mut Request> {
        self.requests
            .iter_mut()
            .find(|request| request.request_id == request_id)
    }

    fn get_stage(start_time: u64) -> RequestState {
        let elapsed = env::block_timestamp() - start_time;

        if start_time == 0 {
            RequestState::NonStarted
        } else if elapsed < COMMIT_MINER_DURATION {
            RequestState::CommitMiners
        } else if elapsed < COMMIT_MINER_DURATION + REVEAL_MINER_DURATION {
            RequestState::RevealMiners
        } else if elapsed
            < COMMIT_MINER_DURATION + REVEAL_MINER_DURATION + COMMIT_VALIDATOR_DURATION
        {
            RequestState::CommitValidators
        } else if elapsed
            < COMMIT_MINER_DURATION
                + REVEAL_MINER_DURATION
                + COMMIT_VALIDATOR_DURATION
                + REVEAL_VALIDATOR_DURATION
        {
            RequestState::RevealValidators
        } else {
            RequestState::Ended
        }
    }

    pub fn hash_miner_answer(self, request_id: String, answer: bool, message: String) -> Hash {
        let miner = env::predecessor_account_id();

        let concatenated_answer = format!("{}{}{}{}", request_id, miner, answer, message);
        let value = env::keccak256(concatenated_answer.as_bytes());

        // Return the hash of the answer
        hex::encode(value)
    }

    pub fn commit_by_miner(&mut self, request_id: String, answer: Hash) -> CommitMinerResult {
        let miner = env::predecessor_account_id();

        if self.get_register_miner(miner.clone()).is_none() {
            log!("Miner not registered: {}", miner);
            return CommitMinerResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return CommitMinerResult::Fail;
        }

        let complete_request: &mut Request = self
            .get_request_by_id(request_id.clone())
            .map_or_else(|| panic!("Request not found"), |request| request);

        assert_eq!(
            Self::get_stage(complete_request.start_time),
            RequestState::CommitMiners,
            "Not at CommitMiners stage"
        );

        if complete_request.miners_proposals.get(&miner).is_some() {
            log!("This miner have a commit answer: {}", miner);
            return CommitMinerResult::Fail;
        }

        let proposal = MinerProposal {
            proposal_hash: answer.clone(),
            answer: false,
            is_revealed: false,
        };

        complete_request.miners_proposals.insert(miner, proposal);

        let commit_miner_log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::CommitMiner(vec![CommitMinerLog { request_id, answer }]),
        };

        env::log_str(&commit_miner_log.to_string());

        CommitMinerResult::Success
    }

    pub fn hash_validator_answer(
        self,
        request_id: String,
        answer: Vec<AccountId>,
        message: String,
    ) -> Hash {
        let validator = env::predecessor_account_id();

        require!(answer.len() == 10, "Invalid answer");

        let mut concatenated_answer: Vec<u8> = Vec::new();

        concatenated_answer.extend_from_slice(request_id.as_bytes());
        concatenated_answer.extend_from_slice(validator.as_bytes());

        let value: Vec<u8> = answer
            .iter()
            .flat_map(|id| id.as_bytes())
            .copied()
            .collect();
        concatenated_answer.extend_from_slice(&value);
        concatenated_answer.extend_from_slice(message.as_bytes());

        let value = env::keccak256(&concatenated_answer);

        // Return the hash of the answer
        hex::encode(value)
    }

    pub fn commit_by_validator(
        &mut self,
        request_id: String,
        answer: Hash,
    ) -> CommitValidatorResult {
        let validator = env::predecessor_account_id();

        if self.get_register_validator(validator.clone()).is_none() {
            log!("Validator is not registered: {}", validator);
            return CommitValidatorResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return CommitValidatorResult::Fail;
        }

        let complete_request: &mut Request = self
            .get_request_by_id(request_id.clone())
            .map_or_else(|| panic!("Request not found"), |request| request);

        assert_eq!(
            Self::get_stage(complete_request.start_time),
            RequestState::CommitValidators,
            "Not at CommitValidator stage"
        );

        if complete_request
            .validators_proposals
            .get(&validator)
            .is_some()
        {
            log!("This validator have a commit answer: {}", validator);
            return CommitValidatorResult::Fail;
        }

        let proposal = ValidatorProposal {
            proposal_hash: answer.clone(),
            is_revealed: false,
            miner_addresses: Vec::new(),
        };

        complete_request
            .validators_proposals
            .insert(validator, proposal);

        let commit_validator_log = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::CommitValidator(vec![CommitValidatorLog {
                request_id,
                answer,
            }]),
        };
        env::log_str(&commit_validator_log.to_string());

        CommitValidatorResult::Success
    }

    pub fn reveal_by_miner(
        &mut self,
        request_id: String,
        answer: bool,
        message: String,
    ) -> RevealMinerResult {
        let miner = env::predecessor_account_id();

        if self.get_register_miner(miner.clone()).is_none() {
            log!("Miner not registered: {}", miner);
            return RevealMinerResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealMinerResult::Fail;
        }

        let complete_request = self
            .get_request_by_id(request_id.clone())
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
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::RevealMiner(vec![RevealMinerLog {
                request_id,
                answer,
                message,
            }]),
        };

        env::log_str(&reveal_miner_log.to_string());

        RevealMinerResult::Success
    }

    pub fn reveal_by_validator(
        &mut self,
        request_id: String,
        answer: Vec<AccountId>,
        message: String,
    ) -> RevealValidatorResult {
        let validator = env::predecessor_account_id();

        if self.get_register_validator(validator.clone()).is_none() {
            log!("Validator is not registered: {}", validator);
            return RevealValidatorResult::Fail;
        }

        if self.get_request_by_id(request_id.clone()).is_none() {
            log!("Request is not registered: {}", request_id);
            return RevealValidatorResult::Fail;
        }

        let complete_request = self
            .get_request_by_id(request_id.clone())
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

        if answer.len() != 10 {
            log!("Invalid answer");
            return RevealValidatorResult::Fail;
        }

        let mut concatenated_answer: Vec<u8> = Vec::new();

        concatenated_answer.extend_from_slice(request_id.as_bytes());
        concatenated_answer.extend_from_slice(validator.as_bytes());

        let value: Vec<u8> = answer
            .iter()
            .flat_map(|id| id.as_bytes())
            .copied()
            .collect();
        concatenated_answer.extend_from_slice(&value);
        concatenated_answer.extend_from_slice(message.as_bytes());

        let value = env::keccak256(&concatenated_answer);
        let hash_answer = hex::encode(value);

        if save_proposal.proposal_hash != hash_answer {
            log!("Answer don't match");
            return RevealValidatorResult::Fail;
        }

        save_proposal.is_revealed = true;
        let answer_for_log = answer.clone();

        for addresses in answer {
            save_proposal.miner_addresses.push(addresses);
        }

        self.aggregate_votes(request_id.clone(), answer_for_log.clone());

        let reveal_validator_log = EventLog {
            standard: "nep171".to_string(),
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

    fn aggregate_votes(&mut self, request_id: String, answer: Vec<AccountId>) {
        let complete_request = self
            .get_request_by_id(request_id.clone())
            .map_or_else(|| panic!("Request not found"), |request| request);

        for address in answer {
            let count = complete_request
                .validators_votes_to_miner
                .get(&address)
                .unwrap_or(&0);
            complete_request
                .validators_votes_to_miner
                .insert(address, count + 1);
        }
    }
    //TODO: Find in the LookupMap
}
