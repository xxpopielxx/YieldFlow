// Struktury danych związane ze stakingiem użytkowników
// 
// Główne komponenty:
// 1. PayoutSchedule - enum określający harmonogram wypłat
//    - Warianty:
//      * Disabled - brak automatycznych wypłat
//      * Daily - codzienne wypłaty
//      * Weekly(u8) - cotygodniowe (0-6 = niedziela-sobota)
//      * Monthly(u8) - comiesięczne (1-28 dzień miesiąca)
//      * Custom(i64) - niestandardowy interwał w sekundach
//    - Implementuje Default jako Disabled
//
// 2. UserStake - główna struktura przechowująca dane stakingu użytkownika
//    - Pola:
//      * user: Pubkey - klucz użytkownika
//      * msol_amount: u64 - ilość msol w stake'u
//      * base_sol_value: u64 - bazowa wartość w SOL
//      * last_update: i64 - timestamp ostatniej aktualizacji
//      * bump: u8 - wartość bump dla PDA
//      * last_dividend: u64 - ostatnia wypłacona dywidenda
//      * total_dividends: u64 - łączne wypłacone dywidendy
//      * payout_schedule: PayoutSchedule - harmonogram wypłat
//      * next_payout_date: i64 - data następnej wypłaty
//      * min_dividend_amount: u64 - minimalna kwota do wypłaty
//      * auto_claim_enabled: bool - czy auto-wypłata jest włączona
//
//    - Metody:
//      * const LEN - określa rozmiar struktury (32 + 8*5 + 1 + 1 + 8*2 + 1 bajtów)
//
// Uwagi:
// - Struktura jest accountem Anchor (atrybut #[account])
// - Implementuje Default dzięki ręcznej implementacji Default dla PayoutSchedule

use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum PayoutSchedule {
    Disabled,
    Daily,
    Weekly(u8),  // 0-6 (niedziela-sobota)
    Monthly(u8), // 1-28
    Custom(i64), // Interwał w sekundach
}

// Ręczna implementacja Default dla PayoutSchedule
impl Default for PayoutSchedule {
    fn default() -> Self {
        PayoutSchedule::Disabled // Domyślna wartość
    }
}

#[account]
#[derive(Default)] // Domyślna implementacja

pub struct UserStake {
    pub user: Pubkey,
    pub msol_amount: u64,
    pub base_sol_value: u64,
    pub last_update: i64,
    pub bump: u8,
    pub last_dividend: u64,
    pub total_dividends: u64,
    
    // Pola harmonogramu
    pub payout_schedule: PayoutSchedule,
    pub next_payout_date: i64,
    pub min_dividend_amount: u64,
    pub auto_claim_enabled: bool,
}

impl UserStake {
    pub const LEN: usize = 32 + 8*5 + 1 + 1 + 8*2 + 1;
}