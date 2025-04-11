use anchor_lang::prelude::*;

#[account]
pub struct UserStake {
    pub user: Pubkey,
    pub msol_amount: u64,
    pub base_sol_value: u64,
    pub last_update: i64,
    pub bump: u8,
    pub last_dividend: u64,
    pub total_dividends: u64,
}

impl UserStake {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 1 + 8 + 8;
}