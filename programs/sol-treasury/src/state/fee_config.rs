use anchor_lang::prelude::*;

#[account]
pub struct FeeConfig {
    pub fee_holder: Pubkey,
    pub fee_per_thousand: u64,
}
