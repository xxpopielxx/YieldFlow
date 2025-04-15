// Plik obsługujący proces wypłat dywidend
//
// Główne funkcjonalności:
// 1. Wypłata dywidend z mSOL w dwóch trybach:
//    - Auto: automatyczna wypłata zgodna z harmonogramem
//    - Manual: ręczna wypłata pomijająca ograniczenia
//
// 2. Logika walidacji:
//    - Sprawdza czy wypłata jest możliwa
//    - Weryfikuje warunki harmonogramu (dla trybu auto)
//    - Sprawdza minimalne kwoty wypłat
//
// 3. Bezpieczeństwo:
//    - Wymaga podpisu użytkownika
//    - Weryfikuje zgodność kont
//    - Chroni przed nadużyciami przez sprawdzanie warunków
//
// Struktury:
// - ClaimDividend: Konta wymagane do wypłaty dywidendy
// - ClaimMode: Enum określający tryb wypłaty (Auto/Manual)
//
// Proces wypłaty:
// 1. Obliczenie aktualnej wartości dywidendy
// 2. Walidacja zgodnie z trybem
// 3. Wykonanie wypłaty przez integrację z Marinade
// 4. Aktualizacja stanu użytkownika
// 5. Aktualizacja harmonogramu (dla trybu auto)


use anchor_lang::prelude::*;
use anchor_spl::token::Token;
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
    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum ClaimMode {
    Auto,    // Automatyczna wypłata zgodna z harmonogramem
    Manual,  // Ręczna wypłata (pomija harmonogram)
}

pub fn handler(ctx: Context<ClaimDividend>, mode: ClaimMode) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    // 1. Oblicz dywidendę
    let current_msol_value = marinade::get_msol_rate(
        &ctx.accounts.marinade_program,
        &ctx.accounts.msol_mint
    )?;
    
    let dividend = math::calculate_dividend(
        user_stake.msol_amount,
        user_stake.base_sol_value,
        current_msol_value
    )?;

    // 2. Walidacja wypłaty
    match mode {
        ClaimMode::Auto => {
            require!(
                user_stake.auto_claim_enabled,
                ErrorCode::AutoClaimDisabled
            );
            require!(
                current_timestamp >= user_stake.next_payout_date,
                ErrorCode::PayoutNotDue
            );
            require!(
                dividend >= user_stake.min_dividend_amount,
                ErrorCode::DividendBelowMinimum
            );
        }
        ClaimMode::Manual => {
            // Wymuszona wypłata - pomija warunki harmonogramu
            require!(
                dividend > 0,
                ErrorCode::NoDividendToClaim
            );
        }
    }

    // 3. Wypłata dywidendy
    if dividend > 0 {
        marinade::withdraw_dividend(
            user_stake,
            dividend,
            &ctx.accounts.escrow_account,
            &ctx.accounts.usdc_mint,
            &ctx.accounts.token_program,
            &ctx.accounts.system_program,
        )?;

        // 4. Aktualizacja stanu
        user_stake.base_sol_value = current_msol_value;
        user_stake.last_update = current_timestamp;
        user_stake.last_dividend = dividend;
        user_stake.total_dividends = user_stake.total_dividends
            .checked_add(dividend)
            .ok_or(ErrorCode::MathOverflow)?;

        // 5. Aktualizacja harmonogramu dla trybu auto
        if let ClaimMode::Auto = mode {
            user_stake.next_payout_date = ScheduleCalculator::calculate_next_payout(
                user_stake.payout_schedule,
                current_timestamp
            )?;
        }

        msg!(
            "Dividend paid: {} SOL (mode: {:?})", 
            dividend, 
            mode
        );
    }

    Ok(())
}