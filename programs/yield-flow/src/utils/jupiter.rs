use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction}
};
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken
};

/// Moduł do integracji z Jupiter DEX Aggregator
pub mod jupiter {
    use super::*;

    // Stałe
    const JUPITER_PROGRAM_ID: Pubkey = pubkey!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"); // Adres programu Jupitera
    const SLIPPAGE_BPS: u64 = 50; // 0.5% dopuszczalnego poślizgu

    /// Wykonuje swap SOL → USDC przez Jupiter DEX Aggregator
    pub fn swap_sol_to_usdc(
        ctx: &Context<Swap>,
        sol_amount: u64, // Ilość SOL w lamports
    ) -> Result<u64> {
        // 1. Pobierz aktualny kurs z Oracla (np. Pyth)
        let usdc_per_sol = get_sol_usdc_rate()?;

        // 2. Oblicz minimalną oczekiwaną ilość USDC (z uwzględnieniem poślizgu)
        let min_out_amount = calculate_min_out(sol_amount, usdc_per_sol)?;

        // 3. Przygotuj dane swapu
        let swap_data = JupiterSwapData {
            amount_in: sol_amount,
            min_amount_out: min_out_amount,
            platform_fee_bps: 0, // Brak dodatkowych opłat
        };

        // 4. Wywołaj CPI do programu Jupiter
        let cpi_program = ctx.accounts.jupiter_program.to_account_info();
        let cpi_accounts = jupiter_ix::Swap {
            user: ctx.accounts.user.to_account_info(),
            input_token_account: ctx.accounts.wsol_account.to_account_info(),
            output_token_account: ctx.accounts.usdc_account.to_account_info(),
            // ... inne wymagane konta
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        jupiter_ix::swap(cpi_ctx, swap_data)?;

        // 5. Zwróć otrzymaną ilość USDC
        Ok(ctx.accounts.usdc_account.amount)
    }

    /// Struktura kont wymaganych do swapu
    #[derive(Accounts)]
    pub struct Swap<'info> {
        /// Portfel użytkownika (signer)
        #[account(mut)]
        pub user: Signer<'info>,

        /// Konto WSOL użytkownika
        #[account(
            mut,
            associated_token::mint = wsol_mint,
            associated_token::authority = user
        )]
        pub wsol_account: Box<Account<'info, TokenAccount>>,

        /// Konto USDC użytkownika
        #[account(
            mut,
            associated_token::mint = usdc_mint,
            associated_token::authority = user
        )]
        pub usdc_account: Box<Account<'info, TokenAccount>>,

        /// Mint WSOL
        #[account(address = spl_token::native_mint::ID)]
        pub wsol_mint: Box<Account<'info, Mint>>,

        /// Mint USDC
        pub usdc_mint: Box<Account<'info, Mint>>,

        /// Program Jupiter
        #[account(address = JUPITER_PROGRAM_ID)]
        pub jupiter_program: Program<'info, Jupiter>,

        /// Program tokenowy
        pub token_program: Program<'info, Token>,

        /// Program Associated Token
        pub associated_token_program: Program<'info, AssociatedToken>,
    }

    // Helper: Pobiera aktualny kurs SOL/USDC
    fn get_sol_usdc_rate() -> Result<u64> {
        // Implementacja integracji z Oracle (np. Pyth Network)
        // W przykładzie stała wartość 100 USDC/SOL
        Ok(100 * 1_000_000) // 100 USDC z dokładnością 6 miejsc
    }

    // Helper: Oblicza minimalną oczekiwaną ilość USDC
    fn calculate_min_out(sol_amount: u64, rate: u64) -> Result<u64> {
        let expected = sol_amount
            .checked_mul(rate)
            .ok_or(ErrorCode::Overflow)?
            .checked_div(1_000_000_000)
            .ok_or(ErrorCode::Overflow)?;

        expected
            .checked_sub(
                expected
                    .checked_mul(SLIPPAGE_BPS)
                    .ok_or(ErrorCode::Overflow)?
                    .checked_div(10_000)
                    .ok_or(ErrorCode::Overflow)?
            )
            .ok_or(ErrorCode::Overflow)
    }

    /// Własne kody błędów
    #[error_code]
    pub enum ErrorCode {
        #[msg("Przekroczono dopuszczalny poślizg")]
        SlippageExceeded,
        #[msg("Błąd obliczeń matematycznych")]
        Overflow,
        #[msg("Brak płynności na rynku")]
        InsufficientLiquidity,
    }
}