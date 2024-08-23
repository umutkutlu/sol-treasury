use anchor_lang::prelude::*;

use crate::state::AccessRoles;
use crate::state::AttackCounter;
use crate::state::AttackData;
use crate::state::PendingAttacks;

#[derive(Accounts)]
pub struct FinalizeAttack<'info> {
    #[account(
        init,
        payer = payer,
        seeds =[b"attack_data_account".as_ref()],
        bump,
        space = 8 + size_of::<AttackData>(),
        constraint = access_roles_account.owner == authority.key()
    )]
    attack_data_account: Account<'info, AttackData>,

    #[account(
        mut,
        seeds =[b"pending_attacks_account".as_ref()],
        bump,
    )]
    pending_attacks_account: Account<'info, PendingAttacks>,

    #[account(
        mut,
        seeds =[b"attack_counter_account".as_ref()],
        bump,
    )]
    attack_counter_account: Account<'info, AttackCounter>,

    #[account(
        seeds =[b"access_roles_account".as_ref()],
        bump,
        constraint = access_roles_account.owner == authority.key()
    )]
    access_roles_account: Account<'info, AccessRoles>,

    #[account(mut)]
    payer: Signer<'info>,
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<FinalizeAttack>) -> Result<()> {
    Ok(())
}
