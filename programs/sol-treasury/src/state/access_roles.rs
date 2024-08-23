use anchor_lang::prelude::*;

#[account]
pub struct AccessRoles {
    pub owner: Pubkey,
}
