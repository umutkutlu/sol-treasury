use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::Treasury;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [crate::constants::TREASURY_SEED],
        bump,
        space = 8 + size_of::<Treasury>(),
    )]
    treasury_account: Account<'info, Treasury>,

    /// CHECK: This is a PDA
    #[account(
        init, 
        payer = payer,
        seeds = [crate::constants::TREASURY_HOLDER_SEED], 
        bump, 
        space = 8
    )]
    treasury_holder_account: AccountInfo<'info>,
    
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeTreasury>) -> Result<()> {
    let treasury_account = &mut ctx.accounts.treasury_account;
    treasury_account.amount = 0;
    Ok(())
}
