use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::FeeHolder;

#[derive(Accounts)]
pub struct InitializeFeeHolder<'info> {
    #[account(
        init,
        payer = payer,
        seeds =[b"fee_holder_account".as_ref()],
        bump,
        space = 8 + size_of::<FeeHolder>(),
    )]
    fee_holder_account: Account<'info, FeeHolder>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<InitializeFeeHolder>) -> Result<()> {
    Ok(())
}
