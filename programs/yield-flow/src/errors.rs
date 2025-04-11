use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Marinade program")]
    InvalidMarinadeProgram,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Dividend too small")]
    DividendTooSmall,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Pandle API error")]
    PandleApiError,
    #[msg("Pandle swap failed")]
    PandleSwapFailed,
    #[msg("Invalid USDC mint")]
    InvalidUsdcMint,
    #[msg("Staking not initialized")]
    StakingNotInitialized,
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    #[msg("Escrow account mismatch")]
    EscrowAccountMismatch,
}