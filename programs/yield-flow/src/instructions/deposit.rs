// Moduł obsługi depozytów mSOL w programie stakingowym
//
// Główne funkcjonalności:
// 1. Przyjmowanie depozytów tokenów mSOL od użytkowników
// 2. Bezpieczne przechowywanie zdeponowanych środków na koncie escrow
// 3. Aktualizacja stanu stake'u użytkownika
//
// Struktura kont:
// - user_stake: Konto stake'u użytkownika (PDA, mutowalne)
//   • Weryfikacja przez seeds ["user-stake", user_key]
//   • Wymagany podpis użytkownika (has_one constraint)
// - user_msol_account: Konto mSOL użytkownika (mutowalne ATA)
// - escrow_account: Konto escrow programu (PDA, mutowalne)
// - msol_mint: Mint tokenów mSOL (tylko do odczytu)
// - config: Konfiguracja programu (tylko do odczytu)
//
// Proces depozytu:
// 1. Transfer mSOL z konta użytkownika do escrow programu:
//    • Wykorzystanie CPI do bezpiecznego transferu
//    • Weryfikacja uprawnień przez program tokenowy
// 2. Aktualizacja stanu użytkownika:
//    • Zwiększenie salda msol_amount
//    • Ustawienie wartości bazowej (base_sol_value)
//
// Zabezpieczenia:
// - Weryfikacja PDA dla konta stake'u i escrow
// - Sprawdzanie overflow matematycznego
// - Wymagany podpis użytkownika
// - Automatyczna weryfikacja kont ATA



use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{UserStake, ProgramConfig};

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// User's stake account
    #[account(
        mut,
        has_one = user,
        seeds = [b"user-stake", user.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    /// User's wallet (signer)
    #[account(mut)]
    pub user: Signer<'info>,

    /// User's mSOL token account
    #[account(
        mut,
        token::mint = msol_mint,
        token::authority = user
    )]
    pub user_msol_account: Account<'info, TokenAccount>,

    /// Escrow account for mSOL
    #[account(
        mut,
        seeds = [b"escrow"],
        bump
    )]
    pub escrow_account: Account<'info, TokenAccount>,

    /// mSOL mint
    pub msol_mint: Account<'info, Mint>,

    /// Program configuration
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    /// Token program
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;

    // 1. Transfer mSOL from user to escrow
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_msol_account.to_account_info(),
        to: ctx.accounts.escrow_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // 2. Update user stake account
    user_stake.msol_amount = user_stake
        .msol_amount
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // Set the base value for the deposit (1 mSOL = current SOL value)
    user_stake.base_sol_value = marinade::get_msol_rate(&ctx.accounts.config)?;

    Ok(())
}