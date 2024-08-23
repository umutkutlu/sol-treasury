use anchor_lang::prelude::*;

use crate::state::FeeHolder;
use crate::state::AccessRoles;

#[derive(Accounts)]
pub struct SetFeeHolder<'info> {
    #[account(
        mut,
        seeds =[b"fee_holder_account".as_ref()],
        bump,
    )]
    fee_holder_account: Account<'info, FeeHolder>,
    
    #[account(
        seeds =[b"access_roles_account".as_ref()],
        bump,
        constraint = access_roles_account.owner == authority.key()
    )]
    access_roles_account: Account<'info, AccessRoles>,
    authority: Signer<'info>,
}

pub fn handler(ctx: Context<SetFeeHolder>, new_fee_holder:Pubkey) -> Result<()> {
    let fee_holder_account = &mut ctx.accounts.fee_holder_account;
    fee_holder_account.fee_holder = new_fee_holder;

    Ok(())
}
