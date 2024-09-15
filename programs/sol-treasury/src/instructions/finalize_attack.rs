use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use std::mem::size_of;

use crate::state::AccessRoles;
use crate::state::AttackCounter;
use crate::state::AttackData;
use crate::state::PendingAttacks;
use crate::state::Treasury;

#[derive(Accounts)]
pub struct FinalizeAttack<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [crate::constants::ATTACK_DATA_SEED, &attack_counter_account.count.to_le_bytes()],
        bump,
        space = 8 + size_of::<AttackData>(),
        constraint = access_roles_account.owner == authority.key()
    )]
    attack_data_account: Account<'info, AttackData>,

    #[account(
        mut,
        seeds = [crate::constants::PENDING_ATTACKS_SEED],
        bump,
    )]
    pending_attacks_account: Account<'info, PendingAttacks>,

    #[account(
        mut,
        seeds = [crate::constants::ATTACK_COUNTER_SEED],
        bump,
    )]
    attack_counter_account: Account<'info, AttackCounter>,

    #[account(
        seeds = [crate::constants::ACCESS_ROLES_SEED],
        bump,
        constraint = access_roles_account.owner == authority.key()
    )]
    access_roles_account: Account<'info, AccessRoles>,

    /// CHECK: This is a PDA
    #[account(
        mut,
        seeds = [crate::constants::TREASURY_HOLDER_SEED], 
        bump, 
    )]
    treasury_holder_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [crate::constants::TREASURY_SEED],
        bump,
    )]
    treasury_account: Account<'info, Treasury>,

    #[account(mut)]
    payer: Signer<'info>,
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FinalizeAttack>, random_number: u64) -> Result<()> {
    let pending_attacks_account = &mut ctx.accounts.pending_attacks_account;
    let attack_data_account = &mut ctx.accounts.attack_data_account;
    let treasury_account = &mut ctx.accounts.treasury_account;
    let treasury_holder_account = &mut ctx.accounts.treasury_holder_account;
    let attack_counter_account = &mut ctx.accounts.attack_counter_account;

    // Get the first pending attack
    let attack_request = pending_attacks_account.array.remove(0);
    // Calculate win chance and reward
    let treasury_amount = treasury_account.amount;
    let attack_amount = attack_request.amount;
    let attack_amount_without_fee = attack_request.amount - attack_request.fee;
    let mut extra_amount = 0;

    if attack_amount > treasury_amount {
        extra_amount = attack_amount - treasury_amount;
        // Transfer extra amount back to the player if any
        let transfer_extra_ix =
            system_instruction::transfer(treasury_holder_account.key, &attack_request.requester, extra_amount);

        anchor_lang::solana_program::program::invoke(
            &transfer_extra_ix,
            &[
                treasury_holder_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    let win_chance = ((attack_amount - extra_amount) * 50) / (treasury_amount);
    let reward = treasury_amount / 2;

    // Determine if the player wins
    let player_wins = (random_number % 100) < win_chance;

    if player_wins {
        // Transfer reward to the player
        let transfer_reward_ix =
            system_instruction::transfer(treasury_holder_account.key, &attack_request.requester, reward);

        anchor_lang::solana_program::program::invoke(
            &transfer_reward_ix,
            &[
                treasury_holder_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Update treasury_account amount
        treasury_account.amount -= reward;
    } else {
        // Add attack amount to treasury_account
        treasury_account.amount += attack_amount_without_fee;
    }

    // Update attack data
    attack_data_account.requester = attack_request.requester;
    attack_data_account.amount = attack_amount;
    attack_data_account.fee = attack_request.fee;

    // Increment attack counter
    attack_counter_account.count += 1;

    Ok(())
}
