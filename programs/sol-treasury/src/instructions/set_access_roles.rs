use anchor_lang::prelude::*;

use crate::state::AccessRoles;

#[derive(Accounts)]
pub struct SetAccessRoles<'info> {
    #[account(
        mut,
        seeds = [crate::constants::ACCESS_ROLES_SEED],
        bump,
        constraint = access_roles_account.owner == authority.key()
    )]
    access_roles_account: Account<'info, AccessRoles>,

    authority: Signer<'info>,
}

pub fn handler(ctx: Context<SetAccessRoles>, new_owner: Pubkey) -> Result<()> {
    let access_roles_account = &mut ctx.accounts.access_roles_account;
    access_roles_account.owner = new_owner;

    Ok(())
}
