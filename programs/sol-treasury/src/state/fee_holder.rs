use anchor_lang::prelude::*;

#[account]
pub struct FeeHolder {
    pub fee_holder: Pubkey,
}
