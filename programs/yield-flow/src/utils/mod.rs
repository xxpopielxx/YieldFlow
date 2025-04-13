// Moduły narzędziowe 
//
// Zawiera zestaw narzędzi pomocniczych do:
// - Harmonogramów wypłat (schedule)
// - Integracji z Marinade Finance (marinade) 
// - Obliczeń matematycznych (math)
//
// Główne komponenty:
//
// 1. Podmoduły funkcjonalne:
//    * schedule.rs - zarządzanie terminami wypłat
//    * marinade.rs - integracja z Marinade Finance:
//      - Pobieranie kursu mSOL/SOL
//      - Operacje na stake'ach
//    * math.rs - obliczenia finansowe:
//      - Dywidendy
//      - Odsetki składane
//
// 2. Stałe matematyczne:
//    * LAMPORTS_PER_SOL - 1_000_000_000 lamportów = 1 SOL
//    * BPS_PER_PERCENT - 100 punktów bazowych = 1%
//    * MAX_BPS - 10_000 = 100% (maksymalna wartość)
//
// 3. Typy danych:
//    * MsolRate - przechowuje kurs wymiany mSOL:
//      - lamports_per_msol: wartość w lamportach SOL
//      - last_updated: timestamp aktualizacji
//
// 4. Reeksportowane funkcje (dostępne bezpośrednio z utils):
//    * get_msol_rate()
//    * withdraw_stake_rewards() 
//    * calculate_dividend()
//    * calculate_compound_interest()

use anchor_lang::AnchorDeserialize;
pub mod schedule;
pub mod marinade;
pub mod math;


// Re-eksport najczęściej używanych funkcji
use crate::utils::marinade::{
    get_msol_rate,
    withdraw_stake_rewards,
    WithdrawRewards,
};



pub use math::{
    calculate_dividend,
    calculate_compound_interest,
};

/// Wspólne stałe matematyczne
pub mod constants {
    /// Liczba lamportów w 1 SOL (1 SOL = 1 miliard lamportów)
    pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
    
    /// Precyzja obliczeń procentowych (1% = 100 punktów bazowych)
    pub const BPS_PER_PERCENT: u16 = 100;
    
    /// Maksymalna precyzja (100% = 10_000 punktów bazowych)
    pub const MAX_BPS: u16 = 10_000;
}

/// Wspólne typy dla modułów utils
pub mod types {
    use anchor_lang::prelude::*;
    
    /// Reprezentacja kursu mSOL/SOL
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct MsolRate {
        /// Wartość 1 mSOL w lamportach SOL
        pub lamports_per_msol: u64,
        /// Czas ostatniej aktualizacji (unix timestamp)
        pub last_updated: i64,
    }
}