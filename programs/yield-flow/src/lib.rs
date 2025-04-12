// Główny moduł programu Marinade Dividend
//
// Struktura programu:
// - errors.rs - definicje kodów błędów
// - instructions.rs - logika głównych instrukcji
// - state.rs - struktury danych programu
// - utils.rs - narzędzia pomocnicze
//
// Główne funkcjonalności:
//
// 1. Inicjalizacja:
//    - initialize_program() - inicjalizacja programu (tylko admin)
//    - initialize_user_stake() - inicjalizacja stake'u użytkownika
//      * Parametry: msol_amount - ilość tokenów mSOL do stake'u
//
// 2. Wypłaty dywidend:
//    - claim_dividend_auto() - automatyczna wypłata zgodna z harmonogramem
//    - claim_dividend_manual() - ręczna wypłata (pomija harmonogram)
//
// 3. Administracja:
//    - update_admin() - aktualizacja konta administratora
//
// Bezpieczeństwo:
// - Wszystkie operacje administracyjne wymagają uprawnień admina
// - Automatyczne wypłaty weryfikują harmonogram i minimalne kwoty
// - Ręczne wypłaty dostępne jako override dla specjalnych przypadkówS


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

    // Automatyczna wypłata (zgodna z harmonogramem)
    pub fn claim_dividend_auto(ctx: Context<ClaimDividend>) -> Result<()> {
        instructions::claim::handler(ctx, ClaimMode::Auto)
    }

    // Ręczna wypłata (pomija harmonogram)
    pub fn claim_dividend_manual(ctx: Context<ClaimDividend>) -> Result<()> {
        instructions::claim::handler(ctx, ClaimMode::Manual)
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
