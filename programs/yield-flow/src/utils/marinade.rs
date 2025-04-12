// Moduł integracji z Marinade Finance
//
// Główne funkcjonalności:
// 1. get_msol_rate() - pobiera aktualny kurs wymiany mSOL/SOL
//    - Parametry:
//      * marinade_state: &mut Account<State> - konto stanu Marinade
//    - Proces:
//      1. Wymusza aktualizację stanu z blockchaina
//      2. Oblicza kurs używając funkcji Marinade calc_msol_from_sol()
//    - Zwraca: kurs w lamportach (1 mSOL = X lamportów SOL)
//
// 2. withdraw_stake_rewards() - wypłaca nagrody stakingowe SOL z Marinade
//    - Parametry:
//      * ctx: &Context<WithdrawRewards> - kontekst z wymaganymi kontami
//      * amount_lamports: u64 - ilość SOL do wypłaty w lamportach
//    - Proces:
//      1. Przygotowuje wszystkie wymagane konta Marinade
//      2. Generuje PDA (Program Derived Address) do podpisywania transakcji
//      3. Wywołuje CPI (Cross-Program Invocation) do programu Marinade
//      4. Wykonuje instrukcję withdraw_stake_rewards
//
// Struktura WithdrawRewards<'info> - wymagane konta:
// - state: Account<State> - główne konto stanu Marinade
// - reserve_pda: AccountInfo - konto rezerwy Marinade (PDA)
// - validator_list: AccountInfo - lista validatorów
// - stake_list: AccountInfo - lista stake'ów
// - msol_mint: Account<Mint> - mint tokenów mSOL
// - treasury_msol_account: AccountInfo - portfel treasury Marinade
// - destination: AccountInfo - docelowy portfel SOL
// - stake_account: AccountInfo - stake account użytkownika
// - destination_stake_account: AccountInfo - docelowy stake account
// - manager_authority: Signer - autoryzacja managera (PDA)
// - marinade_program: Program - program Marinade Finance
// - Konta systemowe: Clock, Token, System, Rent
//
// Stałe:
// - LAMPORTS_PER_SOL: 1_000_000_000 (1 SOL w lamportach)
// - MARINADE_PROGRAM_ID: stały adres programu Marinade
//
// Obsługa błędów:
// - NieprawidłowyStanMarinade - błędne konto stanu
// - BłądObliczaniaKursu - problem z obliczeniem kursu mSOL
// - NieautoryzowanyDostęp - brak autoryzacji managera
//
// Uwagi:
// - Wymaga aktualnego stanu Marinade (reload())
// - Używa CPI do interakcji z programem Marinade
// - Wykorzystuje PDA do podpisywania transakcji
// - Integruje się z systemem stakingowym Marinade

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction}
};


use marinade_finance::{
    instruction as marinade_ix, 
    state::State,
    ID as MARINADE_PROGRAM_ID
};
use anchor_spl::token::{Mint, Token};

/// Moduł pomocniczy do integracji z Marinade Finance
pub mod marinade {
    use super::*;

    // Stała określająca liczbę lamportów w 1 SOL (1 SOL = 1 miliard lamportów)
    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    /// Pobiera aktualny kurs mSOL/SOL w lamportach
    /// np. 1 mSOL = 1.06 SOL → 1_060_000_000 lamportów
    pub fn get_msol_rate(marinade_state: &mut Account<State>) -> Result<u64> {
        // Wymuszenie aktualizacji stanu z blockchaina
        marinade_state.reload()?;
        
        // Obliczenie kursu używając funkcji Marinade
        marinade_state.calc_msol_from_sol(LAMPORTS_PER_SOL)
            .map_err(|_| ErrorCode::BłądObliczaniaKursu.into())
    }

    /// Wypłaca fizyczne SOL z puli nagród stakingowych Marinade
    /// Bez naruszania zdeponowanych mSOL użytkownika
    pub fn withdraw_stake_rewards(
        ctx: &Context<WithdrawRewards>,
        amount_lamports: u64,
    ) -> Result<()> {
        // Przygotowanie wszystkich kont wymaganych przez Marinade
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

        // Generowanie PDA (Program Derived Address) do podpisania transakcji
        let (pda, bump) = Pubkey::find_program_address(
            &[b"treasury", &ctx.accounts.state.key().to_bytes()], // Add the state account's public key as part of the seed
            &MARINADE_PROGRAM_ID
        );
        let seeds = &[b"treasury", &ctx.accounts.state.key().to_bytes(), &[bump]]; // Include the bump seed
        let signer = &[&seeds[..]]; // Correctly construct the signer array

        // Wywołanie instrukcji w Marinade z podpisem PDA
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.marinade_program.to_account_info(),
            cpi_accounts,
            &signer
        );

        marinade_ix::withdraw_stake_rewards(cpi_ctx, amount_lamports)
            .map_err(|e| e.into())
    }

    /// Konta wymagane do wypłaty nagród stakingowych
    #[derive(Accounts)]
    pub struct WithdrawRewards<'info> {
        /// Główne konto stanu Marinade (wymaga aktualizacji)
        #[account(
            mut,
            address = State::id() @ ErrorCode::NieprawidłowyStanMarinade
        )]
        pub state: Account<'info, State>,
        
        /// Konto rezerwy Marinade (PDA)
        #[account(
            mut,
            seeds = [b"reserve"], // Automatycznie generowany seed
            bump,
            seeds::program = MARINADE_PROGRAM_ID
        )]
        pub reserve_pda: AccountInfo<'info>,
        
        /// Lista aktywnych validatorów (tylko do odczytu)
        #[account(mut)]
        pub validator_list: AccountInfo<'info>,
        
        /// Lista stake'ów zarządzanych przez Marinade
        #[account(mut)]
        pub stake_list: AccountInfo<'info>,
        
        /// Konto mintingu tokenów mSOL
        #[account(mut)]
        pub msol_mint: Account<'info, Mint>,
        
        /// Portfel treasury Marinade na mSOL
        #[account(mut)]
        pub treasury_msol_account: AccountInfo<'info>,
        
        /// Docelowy portfel na wypłatę SOL
        #[account(mut)]
        pub destination: AccountInfo<'info>,
        
        /// Stake account powiązany z użytkownikiem
        #[account(mut)]
        pub stake_account: AccountInfo<'info>,
        
        /// Docelowy stake account dla wypłaty
        #[account(mut)]
        pub destination_stake_account: AccountInfo<'info>,
        
        /// Autoryzacja managera (PDA programu)
        #[account(
            seeds = [b"authority"],
            bump
        )]
        pub manager_authority: Signer<'info>,
        
        /// Program Marinade (zweryfikowany adres)
        #[account(address = MARINADE_PROGRAM_ID)]
        pub marinade_program: Program<'info, marinade_finance::program::MarinadeFinance>,
        
        // Konta systemowe
        pub clock: Sysvar<'info, Clock>,
        pub token_program: Program<'info, Token>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

    /// Własne kody błędów dla operacji Marinade
    #[error_code]
    pub enum ErrorCode {
        #[msg("Nieprawidłowe konto stanu Marinade")]
        NieprawidłowyStanMarinade,
        #[msg("Błąd podczas obliczania kursu mSOL")]
        BłądObliczaniaKursu,
        #[msg("Nieautoryzowany dostęp do funkcji managera")]
        NieautoryzowanyDostęp,
    }
}