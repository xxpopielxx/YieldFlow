// Moduł integracji z Sanglass - systemem wypłat na Solanie
//
// Główne funkcjonalności:
// 1. `update_sanglass_program_handler` - Aktualizacja adresu programu Sanglass
// 2. `process_withdrawal_handler` - Główna funkcja wypłat do Sanglass
//
// Bezpieczeństwo:
// - Wszystkie operacje wymagają autoryzacji administratora
// - Ścisła weryfikacja kont przez Anchor

use anchor_lang::prelude::*;
use crate::{state::ProgramConfig, errors::ErrorCode};

/// Konta wymagane do aktualizacji programu Sanglass
#[derive(Accounts)]
pub struct UpdateSanglassProgram<'info> {
    #[account(mut, has_one = admin @ ErrorCode::Unauthorized)]
    pub config: Account<'info, ProgramConfig>,
    
    /// Administrator systemu
    pub admin: Signer<'info>,
    
    /// Nowy adres programu Sanglass
    /// CHECK: Weryfikowany przy użyciu
    pub new_sanglass_program: AccountInfo<'info>,
}

/// Konta wymagane do wypłaty środków
#[derive(Accounts)]
pub struct ProcessWithdrawal<'info> {
    #[account(mut, has_one = admin @ ErrorCode::Unauthorized)]
    pub config: Account<'info, ProgramConfig>,
    
    /// Administrator zatwierdzający wypłatę
    pub admin: Signer<'info>,
    
    /// Konto docelowe w systemie Sanglass
    /// CHECK: Weryfikowane przez Sanglass
    #[account(mut)]
    pub sanglass_destination: AccountInfo<'info>,
    
    /// Vault programu (PDA)
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
        constraint = vault.to_account_info().owner == program_id
    )]
    pub vault: AccountInfo<'info>,
    
    /// Program Sanglass
    /// CHECK: Weryfikowany przez config.sanglass_program
    pub sanglass_program: AccountInfo<'info>,
}

/// Aktualizuje adres programu Sanglass
pub fn update_sanglass_program_handler(
    ctx: Context<UpdateSanglassProgram>,
) -> Result<()> {
    ctx.accounts.config.sanglass_program = ctx.accounts.new_sanglass_program.key();
    msg!("Sanglass program updated to: {}", ctx.accounts.config.sanglass_program);
    Ok(())
}

/// Przetwarza wypłatę środków do Sanglass
pub fn process_withdrawal_handler(
    ctx: Context<ProcessWithdrawal>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);
    
    // CPI do Sanglass
    let transfer_ix = spl_token::instruction::transfer(
        &spl_token::id(),
        &ctx.accounts.vault.key(),
        &ctx.accounts.sanglass_destination.key(),
        &ctx.accounts.admin.key(),
        &[],
        amount,
    )?;
    
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.sanglass_destination.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ],
    )?;
    
    msg!("Withdrawal processed to Sanglass: {} lamports", amount);
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Sanglass program mismatch")]
    SanglassProgramMismatch,
}