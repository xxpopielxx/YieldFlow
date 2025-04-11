use anchor_lang::prelude::*;

#[account]
pub struct ProgramConfig {
    pub admin: Pubkey,
    pub marinade_program: Pubkey,
    pub msol_mint: Pubkey,
    pub pandle_program: Pubkey,
    pub usdc_mint: Pubkey,
    pub fee_account: Pubkey,
    pub bump: u8,
    pub fees_enabled: bool,
    pub fee_rate: u16,
}

impl ProgramConfig {
    pub const LEN: usize = 32 * 5 + 1 + 1 + 2;
}