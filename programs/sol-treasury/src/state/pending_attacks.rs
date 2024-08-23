use anchor_lang::prelude::*;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[account]
pub struct PendingAttacks{
    pub array: Vec<AttackRequest>,
}


#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct AttackRequest {
    pub requester: Pubkey,
    pub amount: u64,
    pub fee: u64,
}
