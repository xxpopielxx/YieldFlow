// YieldFlow - zarządzanie dywidendami od stakowania mSOL
//
// Główne funkcjonalności:
// 1. Inicjalizacja stakingu użytkownika (initialize_user_stake)
// 2. Swap mSOL → USDC (Jupiter)
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


use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use marinade_finance::State as MarinadeState;

mod errors;
mod instructions;
mod state;
mod utils;

declare_id!("D2yN7v2dAhXEyFojzWMH6HxXRxzyGmje7S1Rs9HiQc8Q");

#[program]
pub mod yieldflow {
    use super::*;

    // ========== DEPOZYT I SWAP ========== //

    /// Depozyt SOL -> mSOL przez Marinade
    pub fn deposit_sol(
        ctx: Context<DepositSol>,
        amount_lamports: u64,
    ) -> Result<()> {
        instructions::marinade::deposit_handler(ctx, amount_lamports)
    }

    /// Wymiana mSOL -> USDC przez Jupiter
    pub fn swap_msol_to_usdc(
        ctx: Context<SwapMsolToUsdc>,
        amount: u64,
        slippage_bps: u16,
    ) -> Result<()> {
        instructions::jupiter::swap_handler(ctx, amount, slippage_bps)
    }

    // ========== ZARZĄDZANIE DYWDENDAMI ========== //

    pub fn initialize_user_stake(
        ctx: Context<InitializeUserStake>,
        msol_amount: u64,
    ) -> Result<()> {
        instructions::dividend::initialize_handler(ctx, msol_amount)
    }

    pub fn claim_dividend_auto(ctx: Context<ClaimDividend>) -> Result<()> {
        instructions::dividend::claim_handler(ctx, state::ClaimMode::Auto)
    }

    pub fn claim_dividend_manual(ctx: Context<ClaimDividend>) -> Result<()> {
        instructions::dividend::claim_handler(ctx, state::ClaimMode::Manual)
    }

    // ========== ADMINISTRACJA ========== //

    pub fn initialize_program(
        ctx: Context<InitializeProgram>,
        params: state::ProgramParams,
    ) -> Result<()> {
        instructions::admin::initialize_handler(ctx, params)
    }

    pub fn update_admin(ctx: Context<UpdateAdmin>) -> Result<()> {
        instructions::admin::update_admin_handler(ctx)
    }

    pub fn update_program_params(
        ctx: Context<UpdateProgramParams>,
        new_params: state::ProgramParams,
    ) -> Result<()> {
        instructions::admin::update_params_handler(ctx, new_params)
    }
}

// ========== KONTEKSTY DLA INSTRUKCJI ========== //

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub user_sol: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_msol: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub marinade_state: Account<'info, MarinadeState>,
    
    #[account(mut)]
    pub msol_mint: Account<'info, Mint>,
    
    // Marinade liquidity pool accounts
    #[account(mut)]
    pub liq_pool_sol: AccountInfo<'info>,
    #[account(mut)]
    pub liq_pool_msol: Account<'info, TokenAccount>,
    pub liq_pool_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    
    pub marinade_program: Program<'info, marinade_finance::program::MarinadeFinance>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SwapMsolToUsdc<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub user_msol: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,
    
    // Jupiter swap accounts
    #[account(mut)]
    pub jupiter_router: AccountInfo<'info>,
    pub jupiter_program: Program<'info, jupiter_amm::program::Jupiter>,
    
    // Token accounts
    pub msol_mint: Account<'info, Mint>,
    pub usdc_mint: Account<'info, Mint>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

