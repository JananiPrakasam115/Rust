use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PoolStorageAccount {
    pub pool_authority: Pubkey,

    pub total_staked: u64,
    pub user_count: u64,
    pub rewards_per_token: u64,
    pub is_initialized: bool,
}

impl PoolStorageAccount {
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
