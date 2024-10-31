use near_sdk::{test_utils::VMContextBuilder, testing_env, AccountId, NearToken};

use super::constants::{DEFAULT_DEPOSIT, DEFAULT_TIMESTAMP};

pub struct Environment {
    account_id: AccountId,
    block_timestamp: u64,
    attached_deposit: NearToken,
}

impl Environment {
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_account(account_id: AccountId) -> Self {
        Self {
            account_id,
            block_timestamp: DEFAULT_TIMESTAMP,
            attached_deposit: DEFAULT_DEPOSIT,
        }
    }
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_block_timestamp(mut self, block_timestamp: u64) -> Self {
        self.block_timestamp = block_timestamp;
        self
    }
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_attached_deposit(mut self, attached_deposit: NearToken) -> Self {
        self.attached_deposit = attached_deposit;
        self
    }

    pub fn create(self) {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(self.account_id)
            .block_timestamp(self.block_timestamp)
            .attached_deposit(self.attached_deposit);

        testing_env!(builder.build());
    }
}
