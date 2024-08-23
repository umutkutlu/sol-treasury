use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::Treasury;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        payer = payer,
        seeds =[b"treasury_account".as_ref()],
        bump,
        space = 8 + size_of::<Treasury>(),
    )]
    treasury_account: Account<'info, Treasury>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<InitializeTreasury>) -> Result<()> {
    Ok(())
}
