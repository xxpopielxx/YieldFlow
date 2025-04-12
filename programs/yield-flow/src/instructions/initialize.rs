// Plik implementujący inicjalizację stakingu użytkownika
//
// Główne funkcje:
// - Tworzy nowe konto UserStake dla użytkownika
// - Inicjalizuje podstawowe wartości stakingu
// - Weryfikuje poprawność programu Marinade
//
// Struktury:
// - InitializeUserStake: Konta wymagane do inicjalizacji
//   * user_stake: Nowe konto stakingowe (PDA)
//   * user: Podpisujący użytkownik
//   * config: Globalna konfiguracja programu
//   * marinade_program: Weryfikowany program Marinade
//
// Bezpieczeństwo:
// - Wymaga podpisu użytkownika
// - Weryfikuje zgodność programu Marinade
// - Konto inicjalizowane jako PDA z seedem "user-stake"

use anchor_lang::prelude::*;
use crate::{state::{UserStake, ProgramConfig}, errors::ErrorCode};

#[derive(Accounts)]
pub struct InitializeUserStake<'info> {
    #[account(
        init,
        payer = user,
        space = UserStake::LEN,
        seeds = [b"user-stake", user.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.marinade_program == marinade_program.key() @ ErrorCode::InvalidMarinadeProgram
    )]
    pub config: Account<'info, ProgramConfig>,
    
    /// CHECK: Verified by constraint
    pub marinade_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeUserStake>, msol_amount: u64) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    user_stake.user = ctx.accounts.user.key();
    user_stake.msol_amount = msol_amount;
    user_stake.base_sol_value = 1_000_000;
    user_stake.last_update = Clock::get()?.unix_timestamp;
    user_stake.bump = ctx.bumps.user_stake; // Correct bump access
    
    msg!("User stake initialized with bump: {}", user_stake.bump);
    Ok(())
}
