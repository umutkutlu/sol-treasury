use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::FeeConfig;

#[derive(Accounts)]
pub struct InitializeFeeConfig<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [crate::constants::FEE_CONFIG_SEED],
        bump,
        space = 8 + size_of::<FeeConfig>(),
    )]
    fee_config_account: Account<'info, FeeConfig>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeFeeConfig>,
    initial_fee_holder: Pubkey,
    initial_fee_per_thousand: u64,
) -> Result<()> {
    let fee_config_account = &mut ctx.accounts.fee_config_account;
    fee_config_account.fee_holder = initial_fee_holder;
    fee_config_account.fee_per_thousand = initial_fee_per_thousand;
    Ok(())
}
