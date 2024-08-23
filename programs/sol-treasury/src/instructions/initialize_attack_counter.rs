use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::AttackCounter;

#[derive(Accounts)]
pub struct InitializeAttackCounter<'info> {
    #[account(
        init,
        payer = payer,
        seeds =[b"attack_counter_account".as_ref()],
        bump,
        space = 8 + size_of::<AttackCounter>(),
    )]
    attack_counter_account: Account<'info, AttackCounter>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeAttackCounter>) -> Result<()> {
    let attack_counter_account = &mut ctx.accounts.attack_counter_account;
    attack_counter_account.count = 0;

    Ok(())
}
