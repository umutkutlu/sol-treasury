use anchor_lang::prelude::*;

#[account]
pub struct AttackCounter {
    pub count: u64,
}
