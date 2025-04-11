//! Moduły narzędziowe dla programu Marinade Dividend
//!
//! Zawiera pomocnicze funkcje matematyczne i integracyjne

pub mod marinade;
pub mod math;

// Re-eksport najczęściej używanych funkcji
pub use marinade::{
    get_msol_rate,
    withdraw_stake_rewards,
    WithdrawRewards
};

pub use math::{
    calculate_dividend,
    calculate_compound_interest,
    calculate_fee,
    calculate_percentage,
    apr_to_daily_rate
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