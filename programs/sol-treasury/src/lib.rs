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

    pub fn initialize_fee_holder(ctx: Context<InitializeFeeHolder>) -> Result<()> {
        initialize_fee_holder::handler(ctx)
    }

    pub fn initialize_access_roles(
        ctx: Context<InitializeAccessRoles>,
        initial_owner: Pubkey,
    ) -> Result<()> {
        initialize_access_roles::handler(ctx, initial_owner)
    }

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        initialize_treasury::handler(ctx)
    }

    pub fn initialize_pending_attacks(ctx: Context<InitializePendingAttacks>) -> Result<()> {
        initialize_pending_attacks::handler(ctx)
    }

    pub fn set_access_roles(ctx: Context<SetAccessRoles>, new_owner: Pubkey) -> Result<()> {
        set_access_roles::handler(ctx, new_owner)
    }

    pub fn set_fee_holder(ctx: Context<SetFeeHolder>, new_fee_holder: Pubkey) -> Result<()> {
        set_fee_holder::handler(ctx, new_fee_holder)
    }

    pub fn request_attack(ctx: Context<RequestAttack>) -> Result<()> {
        request_attack::handler(ctx)
    }
}
