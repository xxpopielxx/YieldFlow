// Moduł integracji z Marinade Finance - protokołem liquid stakingu na Solanie
//
// Główne funkcjonalności:
// 1. Depozyt SOL -> mSOL (token stakingowy Marinade)
// 2. Wypłata nagród stakingowych
// 3. Pobieranie aktualnego kursu wymiany
//
// Kluczowe komponenty:
// - `deposit_sol()`: Konwersja SOL do mSOL z użyciem CPI (Cross-Program Invocation)
// - `withdraw_stake_rewards()`: Wypłata nagród stakingowych w SOL
// - `get_msol_rate()`: Pobieranie aktualnego kursu mSOL/SOL
//
// Struktury kont:
// - `DepositSol`: Konta wymagane do depozytu SOL
// - `WithdrawRewards`: Konta wymagane do wypłaty nagród
//
// Bezpieczeństwo:
// - Wszystkie operacje wymagają weryfikacji kont Marinade
// - Użycie PDA (Program Derived Address) do podpisywania transakcji
// - Ścisła walidacja uprawnień
//
// Stałe:
// - LAMPORTS_PER_SOL: 1_000_000_000 (1 SOL w lamportach)
// - MARINADE_PROGRAM_ID: Stały adres programu Marinade Finance

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction}
};
use marinade_finance::{
    instruction as marinade_ix, 
    state::State,
    ID as MARINADE_PROGRAM_ID
};
use anchor_spl::token::{Mint, Token, TokenAccount};

/// Moduł pomocniczy do integracji z Marinade Finance
pub mod marinade {
    use super::*;

    // Stała określająca liczbę lamportów w 1 SOL (1 SOL = 1 miliard lamportów)
    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    // ======================== DEPOZYT SOL ======================== //

    /// Wykonuje depozyt SOL -> mSOL poprzez Marinade
    pub fn deposit_sol(
        ctx: &Context<DepositSol>,
        amount_lamports: u64,
    ) -> Result<()> {
        require!(amount_lamports > 0, ErrorCode::InvalidAmount);
    // coś sie wywala przy depozycie chyba odnośnie bibioteki marinade(error: No such field)
        let cpi_accounts = marinade_ix::Deposit {
            state: ctx.accounts.state.to_account_info(),
            msol_mint: ctx.accounts.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: ctx.accounts.liq_pool_sol_leg.to_account_info(),
            liq_pool_msol_leg: ctx.accounts.liq_pool_msol_leg.to_account_info(),
            liq_pool_msol_leg_authority: ctx.accounts.liq_pool_authority.to_account_info(),
            reserve_pda: ctx.accounts.reserve_pda.to_account_info(),
            transfer_from: ctx.accounts.user_sol.to_account_info(),
            mint_to: ctx.accounts.user_msol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            lamports: amount_lamports,
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.marinade_program.to_account_info(),
            cpi_accounts
        );

        marinade_ix::deposit(cpi_ctx, amount_lamports)?;

        Ok(())
    }

    /// Konta wymagane do depozytu SOL
    #[derive(Accounts)]
    pub struct DepositSol<'info> {
        /// Główne konto stanu Marinade
        #[account(mut, address = State::id() @ ErrorCode::NieprawidłowyStanMarinade)]
        pub state: Account<'info, State>,

        /// Użytkownik inicjujący depozyt
        #[account(mut)]
        pub user: Signer<'info>,

        /// SOL użytkownika do zdeponowania
        #[account(mut)]
        pub user_sol: AccountInfo<'info>,

        /// Docelowe konto mSOL użytkownika
        #[account(mut)]
        pub user_msol: Account<'info, TokenAccount>,

        /// Mint tokenów mSOL
        #[account(mut)]
        pub msol_mint: Account<'info, Mint>,

        /// Liquidity pool SOL (PDA)
        #[account(mut)]
        pub liq_pool_sol_leg: AccountInfo<'info>,

        /// Liquidity pool mSOL
        #[account(mut)]
        pub liq_pool_msol_leg: Account<'info, TokenAccount>,

        /// Autoryzacja liquidity pool
        pub liq_pool_authority: AccountInfo<'info>,

        /// Konto rezerwy Marinade
        #[account(mut)]
        pub reserve_pda: AccountInfo<'info>,

        /// Program Marinade
        #[account(address = MARINADE_PROGRAM_ID)]
        pub marinade_program: Program<'info, marinade_finance::program::MarinadeFinance>,

        /// Konta systemowe
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
    }

    // ======================== POZOSTAŁE FUNKCJE ======================== //

    /// Pobiera aktualny kurs mSOL/SOL w lamportach
    pub fn get_msol_rate(marinade_state: &mut Account<State>) -> Result<u64> {
        marinade_state.reload()?;
        marinade_state.calc_msol_from_sol(LAMPORTS_PER_SOL)
            .map_err(|_| ErrorCode::BłądObliczaniaKursu.into())
    }

    /// Wypłaca nagrody stakingowe
    pub fn withdraw_stake_rewards(
        ctx: &Context<WithdrawRewards>,
        amount_lamports: u64,
    ) -> Result<()> {
        let cpi_accounts = marinade_ix::WithdrawStakeRewards {
            state: ctx.accounts.state.to_account_info(),
            reserve_pda: ctx.accounts.reserve_pda.clone(),
            validator_list: ctx.accounts.validator_list.clone(),
            stake_list: ctx.accounts.stake_list.clone(),
            msol_mint: ctx.accounts.msol_mint.to_account_info(),
            treasury_msol_account: ctx.accounts.treasury_msol_account.clone(),
            clock: ctx.accounts.clock.to_account_info(),
            stake_account: ctx.accounts.stake_account.clone(),
            destination_stake_account: ctx.accounts.destination_stake_account.clone(),
            destination: ctx.accounts.destination.to_account_info(),
            manager_authority: ctx.accounts.manager_authority.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let (pda, bump) = Pubkey::find_program_address(
            &[b"treasury", &ctx.accounts.state.key().to_bytes()],
            &MARINADE_PROGRAM_ID
        );
        let signer = &[&[b"treasury", &ctx.accounts.state.key().to_bytes(), &[bump]][..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.marinade_program.to_account_info(),
            cpi_accounts,
            signer
        );

        marinade_ix::withdraw_stake_rewards(cpi_ctx, amount_lamports)
            .map_err(|e| e.into())
    }

    // ======================== STRUKTURY ======================== //

    #[derive(Accounts)]
    pub struct WithdrawRewards<'info> {
        #[account(mut, address = State::id() @ ErrorCode::NieprawidłowyStanMarinade)]
        pub state: Account<'info, State>,
        
        #[account(mut, seeds = [b"reserve"], bump, seeds::program = MARINADE_PROGRAM_ID)]
        pub reserve_pda: AccountInfo<'info>,
        
        #[account(mut)]
        pub validator_list: AccountInfo<'info>,
        
        #[account(mut)]
        pub stake_list: AccountInfo<'info>,
        
        #[account(mut)]
        pub msol_mint: Account<'info, Mint>,
        
        #[account(mut)]
        pub treasury_msol_account: AccountInfo<'info>,
        
        #[account(mut)]
        pub destination: AccountInfo<'info>,
        
        #[account(mut)]
        pub stake_account: AccountInfo<'info>,
        
        #[account(mut)]
        pub destination_stake_account: AccountInfo<'info>,
        
        #[account(seeds = [b"authority"], bump)]
        pub manager_authority: Signer<'info>,
        
        #[account(address = MARINADE_PROGRAM_ID)]
        pub marinade_program: Program<'info, marinade_finance::program::MarinadeFinance>,
        
        pub clock: Sysvar<'info, Clock>,
        pub token_program: Program<'info, Token>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

}