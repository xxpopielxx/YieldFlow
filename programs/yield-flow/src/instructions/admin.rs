// Plik zawierający instrukcje administracyjne programu
//
// Główne funkcjonalności:
// 1. Inicjalizacja konfiguracji programu
//    - Tworzy główne konto konfiguracyjne
//    - Ustawia administratora programu
//    - Zapamiętuje kluczowe adresy (program Marinade, mint mSOL)
//
// 2. Zarządzanie uprawnieniami admina
//    - Pozwala na zmianę administratora programu
//    - Wymaga podpisu obecnego admina
//
// Struktury:
// - InitializeProgram: Konta wymagane do inicjalizacji
// - UpdateAdmin: Konta wymagane do zmiany admina
//
// Funkcje handlerów:
// - initialize_program_handler: Wykonuje inicjalizację
// - update_admin_handler: Aktualizuje administratora
//
// Bezpieczeństwo:
// - Wszystkie operacje wymagają podpisu admina
// - Inicjalizacja może nastąpić tylko raz
// - Zmiana admina wymaga podpisu obecnego admina

use anchor_lang::prelude::*;
use crate::{state::ProgramConfig, errors::ErrorCode};

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(init, payer = admin, space = ProgramConfig::LEN)]
    pub config: Account<'info, ProgramConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub marinade_program: AccountInfo<'info>,
    pub msol_mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, ProgramConfig>,
    pub admin: Signer<'info>,
    pub new_admin: SystemAccount<'info>,
}

pub fn initialize_program_handler(
    ctx: Context<InitializeProgram>,
    bump: u8,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.marinade_program = ctx.accounts.marinade_program.key();
    config.msol_mint = ctx.accounts.msol_mint.key();
    config.bump = bump;
    Ok(())
}

pub fn update_admin_handler(ctx: Context<UpdateAdmin>) -> Result<()> {
    ctx.accounts.config.admin = ctx.accounts.new_admin.key();
    Ok(())
}