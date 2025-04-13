// YieldFlow - zarządzanie dywidendami od stakowania mSOL
//
// Główne funkcjonalności:
// 1. Inicjalizacja stakingu użytkownika (initialize_user_stake)
// 2. Automatyczne i manualne pobieranie dywidend (claim_dividend_auto/claim_dividend_manual)
// 3. Funkcje administracyjne:
//    - Inicjalizacja programu (initialize_program)
//    - Aktualizacja administratora (update_admin)
//
// Struktura modułów:
// - errors: Definicje błędów programu
// - instructions: Logika głównych instrukcji
// - state: Struktury danych programu
// - utils: Narzędzia pomocnicze

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

    pub fn claim_dividend_auto(ctx: Context<ClaimDividend>) -> Result<()> {
        instructions::claim::handler(ctx, ClaimMode::Auto)
    }

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
