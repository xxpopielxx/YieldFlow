// Ten plik zawiera funkcjonalności związane z aktualizacją konfiguracji programu Pandle.
//
// Główne komponenty:
// 1. `update_pandle_program_handler` - Umożliwia administratorowi aktualizację adresu programu Pandle
//    - Wymagania:
//      - Podpis administratora
//      - Modyfikowalny dostęp do konfiguracji programu
//      - Informacje o nowym koncie programu Pandle
//    - Efekty:
//      - Aktualizuje pole `pandle_program` w ProgramConfig
//      - Loguje nowy adres programu
//
// 2. `update_escrow_handler` - Umożliwia aktualizację konta escrow (depozytowego)
//    - Uwaga: W rzeczywistości escrow jest PDA (Program Derived Address),
//      więc ta funkcja może być niepotrzebna lub służyć tylko do aktualizacji referencji
//    - Obecnie tylko loguje informację o aktualizacji
//
// Struktury kont:
// - `UpdatePandleProgram` - Zawiera konta potrzebne do aktualizacji programu Pandle
// - `UpdateEscrow` - Zawiera konta potrzebne do aktualizacji escrow
//
// Bezpieczeństwo:
// - Wszystkie operacje wymagają weryfikacji podpisu administratora
// - Konta są odpowiednio sprawdzane przez atrybuty Anchor (has_one, mut)


use anchor_lang::prelude::*;
use crate::{state::ProgramConfig, errors::ErrorCode};

#[derive(Accounts)]
pub struct UpdatePandleProgram<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, ProgramConfig>,
    pub admin: Signer<'info>,
    /// CHECK: Will be verified when used
    pub new_pandle_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateEscrow<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, ProgramConfig>,
    pub admin: Signer<'info>,
    /// CHECK: Will be verified by seeds when used
    pub new_escrow: AccountInfo<'info>,
}

pub fn update_pandle_program_handler(
    ctx: Context<UpdatePandleProgram>,
) -> Result<()> {
    ctx.accounts.config.pandle_program = ctx.accounts.new_pandle_program.key();
    msg!("Pandle program updated to: {}", ctx.accounts.config.pandle_program);
    Ok(())
}

pub fn update_escrow_handler(
    ctx: Context<UpdateEscrow>,
) -> Result<()> {
    // W rzeczywistości escrow jest PDA, więc ta funkcja może być niepotrzebna
    // lub może aktualizować tylko referencję do escrow
    msg!("Escrow account reference updated");
    Ok(())
}