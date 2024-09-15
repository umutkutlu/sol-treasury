use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::PendingAttacks;

#[derive(Accounts)]
pub struct InitializePendingAttacks<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [crate::constants::PENDING_ATTACKS_SEED],
        bump,
        space = 8 + size_of::<PendingAttacks>(),
    )]
    pending_attacks_account: Account<'info, PendingAttacks>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<InitializePendingAttacks>) -> Result<()> {
    Ok(())
}
