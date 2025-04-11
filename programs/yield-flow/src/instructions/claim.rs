use anchor_lang::prelude::*;
use crate::{state::{UserStake, ProgramConfig}, utils::{marinade, math}, errors::ErrorCode};

#[derive(Accounts)]
pub struct ClaimDividend<'info> {
    #[account(mut, has_one = user)]
    pub user_stake: Account<'info, UserStake>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"escrow"], bump)]
    pub escrow_account: AccountInfo<'info>,

    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, ProgramConfig>,
    pub marinade_program: AccountInfo<'info>,
    pub msol_mint: AccountInfo<'info>,
    
    #[account(mut)]
    pub usdc_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub pandle_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<ClaimDividend>) -> Result<()> {
    let current_msol_value = marinade::get_current_msol_value(
        &ctx.accounts.marinade_program,
        &ctx.accounts.msol_mint
    )?;
    
    let dividend = math::calculate_dividend(
        ctx.accounts.user_stake.msol_amount,
        ctx.accounts.user_stake.base_sol_value,
        current_msol_value
    )?;
    
    marinade::withdraw_dividend(
        &ctx.accounts.user_stake,
        dividend,
        &ctx.accounts.escrow_account,
        &ctx.accounts.usdc_mint,
        &ctx.accounts.token_program,
        &ctx.accounts.pandle_program,
        &ctx.accounts.system_program,
    )?;
    
    ctx.accounts.user_stake.base_sol_value = current_msol_value;
    ctx.accounts.user_stake.last_update = Clock::get()?.unix_timestamp;
    Ok(())
}