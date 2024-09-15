use anchor_lang::prelude::*;

use crate::state::AccessRoles;
use crate::state::FeeConfig;

#[derive(Accounts)]
pub struct SetFeeConfig<'info> {
    #[account(
        mut,
        seeds = [crate::constants::FEE_CONFIG_SEED],
        bump,
    )]
    fee_config_account: Account<'info, FeeConfig>,

    #[account(
        seeds = [crate::constants::ACCESS_ROLES_SEED],
        bump,
        constraint = access_roles_account.owner == authority.key()
    )]
    access_roles_account: Account<'info, AccessRoles>,
    authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<SetFeeConfig>,
    new_fee_holder: Pubkey,
    new_fee_per_thousand: u64,
) -> Result<()> {
    let fee_config_account = &mut ctx.accounts.fee_config_account;
    fee_config_account.fee_holder = new_fee_holder;
    fee_config_account.fee_per_thousand = new_fee_per_thousand;

    Ok(())
}
