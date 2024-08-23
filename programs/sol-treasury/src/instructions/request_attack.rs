use anchor_lang::prelude::*;

use crate::state::PendingAttacks;

#[derive(Accounts)]
pub struct RequestAttack<'info> {
    #[account(
        mut,
        seeds =[b"pending_attacks_account".as_ref()],
        bump,
    )]
    pending_attacks_account: Account<'info, PendingAttacks>,

    #[account(mut)]
    player: Signer<'info>,
}

pub fn handler(_ctx: Context<RequestAttack>) -> Result<()> {
    Ok(())
}
