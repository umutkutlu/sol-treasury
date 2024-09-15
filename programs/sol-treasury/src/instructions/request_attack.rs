use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

use crate::state::{PendingAttacks, AttackRequest, FeeConfig};

#[derive(Accounts)]
pub struct RequestAttack<'info> {
    #[account(
        mut,
        seeds = [crate::constants::PENDING_ATTACKS_SEED],
        bump,
    )]
    pending_attacks_account: Account<'info, PendingAttacks>,

    /// CHECK: This is a PDA
    #[account(
        mut,
        seeds = [crate::constants::TREASURY_HOLDER_SEED], 
        bump, 
    )]
    treasury_holder_account: AccountInfo<'info>,

    #[account(
        seeds = [crate::constants::FEE_CONFIG_SEED],
        bump,
    )]
    fee_config_account: Account<'info, FeeConfig>,

    #[account(mut)]
    player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RequestAttack>, attack_amount: u64) -> Result<()> {
    let from_account = &ctx.accounts.player;
    let treasury_holder_account = &ctx.accounts.treasury_holder_account;
    let fee_config = &ctx.accounts.fee_config_account;

    // Calculate fee
    let fee = (attack_amount * fee_config.fee_per_thousand + 999) / 1000;
    let attack_amount_without_fee = attack_amount - fee;

    // Transfer to treasury
    let transfer_to_treasury_ix = system_instruction::transfer(
        from_account.key,
        treasury_holder_account.key,
        attack_amount_without_fee,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_to_treasury_ix,
        &[
            from_account.to_account_info(),
            treasury_holder_account.clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Transfer fee to fee holder
    let transfer_fee_ix = system_instruction::transfer(
        from_account.key,
        &fee_config.fee_holder,
        fee,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_fee_ix,
        &[
            from_account.to_account_info(),
            // TODO: check if to account needed here,
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Create new AttackRequest
    let new_attack_request = AttackRequest {
        requester: *from_account.key,
        amount: attack_amount,
        fee,
    };

    // Add the new AttackRequest to the pending_attacks_account
    ctx.accounts.pending_attacks_account.array.push(new_attack_request);

    Ok(())
}