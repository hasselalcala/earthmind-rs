use near_sdk::NearToken;

pub const TWO_MINUTES: u64 = 2 * 60 * 1_000_000_000; // 2 minutes in nanoseconds
pub const COMMIT_MINER_DURATION: u64 = TWO_MINUTES;
pub const REVEAL_MINER_DURATION: u64 = TWO_MINUTES;
pub const COMMIT_VALIDATOR_DURATION: u64 = TWO_MINUTES;
pub const REVEAL_VALIDATOR_DURATION: u64 = TWO_MINUTES;
pub const MIN_MINER_STAKE: NearToken = NearToken::from_near(1); // 1 NEAR
pub const MIN_VALIDATOR_STAKE: NearToken = NearToken::from_near(10); // 10 NEAR
pub const PROTOCOL_REGISTRATION_FEE: NearToken = NearToken::from_near(5);
