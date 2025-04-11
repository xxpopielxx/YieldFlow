pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("D2yN7v2dAhXEyFojzWMH6JxXRxzyGmje7S1Rs9HiQc8Q");

#[program]
pub mod marinade_dividend {
    use super::*;

    pub fn initialize_user_stake(
        ctx: Context<InitializeUserStake>,
        msol_amount: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, msol_amount)
    }

    pub fn claim_dividend(ctx: Context<ClaimDividend>) -> Result<()> {
        instructions::claim::handler(ctx)
    }

    pub fn initialize_program(
        ctx: Context<InitializeProgram>,
        bump: u8,
    ) -> Result<()> {
        instructions::admin::initialize_program_handler(ctx, bump)
    }

    pub fn update_admin(ctx: Context<UpdateAdmin>) -> Result<()> {
        instructions::admin::update_admin_handler(ctx)
    }
}
