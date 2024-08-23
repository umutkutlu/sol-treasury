use anchor_lang::prelude::*;

#[account]
pub struct AttackData{
    pub requester: Pubkey,
    pub amount: u64,
    pub fee: u64,
}
