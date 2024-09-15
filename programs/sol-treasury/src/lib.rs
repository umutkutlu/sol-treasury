pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FiGQhpLnHifcBDqNqPuiMmRYuur83RVMAYs3i9T4KcvV");

#[program]
pub mod sol_treasury {
    use super::*;

    pub fn initialize_access_roles(
        ctx: Context<InitializeAccessRoles>,
        initial_owner: Pubkey,
    ) -> Result<()> {
        initialize_access_roles::handler(ctx, initial_owner)
    }

    pub fn initialize_attack_counter(ctx: Context<InitializeAttackCounter>) -> Result<()> {
        initialize_attack_counter::handler(ctx)
    }

    pub fn initialize_fee_config(
        ctx: Context<InitializeFeeConfig>,
        initial_fee_holder: Pubkey,
        initial_fee_per_thousand: u64,
    ) -> Result<()> {
        initialize_fee_config::handler(ctx, initial_fee_holder, initial_fee_per_thousand)
    }

    pub fn initialize_pending_attacks(ctx: Context<InitializePendingAttacks>) -> Result<()> {
        initialize_pending_attacks::handler(ctx)
    }

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        initialize_treasury::handler(ctx)
    }

    pub fn finalize_attack(ctx: Context<FinalizeAttack>, random_number: u64) -> Result<()> {
        finalize_attack::handler(ctx, random_number)
    }

    pub fn request_attack(ctx: Context<RequestAttack>, attack_amount: u64) -> Result<()> {
        request_attack::handler(ctx, attack_amount)
    }

    pub fn set_access_roles(ctx: Context<SetAccessRoles>, new_owner: Pubkey) -> Result<()> {
        set_access_roles::handler(ctx, new_owner)
    }

    pub fn set_fee_config(
        ctx: Context<SetFeeConfig>,
        new_fee_holder: Pubkey,
        new_fee_per_thousand: u64,
    ) -> Result<()> {
        set_fee_config::handler(ctx, new_fee_holder, new_fee_per_thousand)
    }
}
