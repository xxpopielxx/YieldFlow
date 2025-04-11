use anchor_lang::prelude::*;
use crate::{
    state::{UserStake, ProgramConfig},
    utils::{marinade, math, schedule::ScheduleCalculator},
    errors::ErrorCode
};

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
    
    /// CHECK: Verified by Marinade program
    pub marinade_program: AccountInfo<'info>,
    
    /// CHECK: Verified by Marinade program
    pub msol_mint: AccountInfo<'info>,
    
    #[account(mut)]
    /// CHECK: Verified by token program
    pub usdc_mint: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    
    /// CHECK: Verified by Pandle program
    pub pandle_program: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum ClaimMode {
    Automatic, // Tylko jeśli wymagane przez harmonogram
    Force,     // Wymuś wypłatę niezależnie od harmonogramu
}

pub fn handler(ctx: Context<ClaimDividend>, mode: ClaimMode) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    let clock = Clock::get()?;
    
    // 1. Oblicz aktualną wartość mSOL i dywidendę
    let current_msol_value = marinade::get_current_msol_value(
        &ctx.accounts.marinade_program,
        &ctx.accounts.msol_mint
    )?;
    
    let dividend = math::calculate_dividend(
        user_stake.msol_amount,
        user_stake.base_sol_value,
        current_msol_value
    )?;
    
    // 2. Sprawdź czy wypłata jest wymagana/dozwolona
    let should_payout = match mode {
        ClaimMode::Automatic => {
            ScheduleCalculator::should_payout(
                user_stake,
                dividend,
                clock.unix_timestamp
            )?
        }
        ClaimMode::Force => true,
    };
    
    require!(should_payout, ErrorCode::PayoutNotDue);
    
    // 3. Wykonaj wypłatę jeśli spełnione warunki
    if dividend > 0 {
        marinade::withdraw_dividend(
            user_stake,
            dividend,
            &ctx.accounts.escrow_account,
            &ctx.accounts.usdc_mint,
            &ctx.accounts.token_program,
            &ctx.accounts.pandle_program,
            &ctx.accounts.system_program,
        )?;
        
        // 4. Zaktualizuj stan użytkownika
        user_stake.base_sol_value = current_msol_value;
        user_stake.last_update = clock.unix_timestamp;
        user_stake.last_dividend = dividend;
        user_stake.total_dividends = user_stake.total_dividends
            .checked_add(dividend)
            .ok_or(ErrorCode::MathOverflow)?;
        
        // 5. Aktualizuj następną datę wypłaty dla trybu automatycznego
        if matches!(mode, ClaimMode::Automatic) {
            user_stake.next_payout_date = ScheduleCalculator::calculate_next_payout(
                user_stake.payout_schedule,
                clock.unix_timestamp
            )?;
        }
        
        msg!("Dividend paid: {} SOL", dividend);
    }
    
    Ok(())
}