use anchor_lang::prelude::*;

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
#[derive(Default)] // Teraz może być używany, bo PayoutSchedule implementuje Default
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