use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub amount: u64,
}
