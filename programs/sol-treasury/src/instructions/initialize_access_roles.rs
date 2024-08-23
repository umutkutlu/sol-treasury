use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::AccessRoles;

#[derive(Accounts)]
pub struct InitializeAccessRoles<'info> {
    #[account(
        init,
        payer = payer,
        seeds =[b"access_roles_account".as_ref()],
        bump,
        space = 8 + size_of::<AccessRoles>(),
    )]
    access_roles_account: Account<'info, AccessRoles>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeAccessRoles>, initial_owner: Pubkey) -> Result<()> {
    let access_roles_account = &mut ctx.accounts.access_roles_account;
    access_roles_account.owner = initial_owner;

    Ok(())
}
