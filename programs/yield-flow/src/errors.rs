use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // Błędy ogólne
    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid timestamp")]
    InvalidTimestamp,

    // Błędy Marinade
    #[msg("Invalid Marinade program")]
    InvalidMarinadeProgram,

    #[msg("Invalid Marinade state account")]
    InvalidMarinadeState,

    #[msg("Failed to calculate mSOL rate")]
    MsolRateCalculationError,

    // Błędy wypłat dywidend
    #[msg("Dividend amount too small")]
    DividendTooSmall,

    #[msg("No dividend available to claim")]
    NoDividendToClaim,

    #[msg("Dividend below minimum threshold")]
    DividendBelowMinimum,

    #[msg("Payout is not due yet")]
    PayoutNotDue,

    #[msg("Auto claim is disabled")]
    AutoClaimDisabled,

    // Błędy Pandle
    #[msg("Pandle API error")]
    PandleApiError,

    #[msg("Pandle swap failed")]
    PandleSwapFailed,
    
    // Błędy kont
    #[msg("Invalid USDC mint")]
    InvalidUsdcMint,

    #[msg("Staking not initialized")]
    StakingNotInitialized,

    #[msg("Escrow account mismatch")]
    EscrowAccountMismatch,

    #[msg("Invalid account configuration")]
    InvalidAccountConfig,

    //inne błędy
    #[msg("Invalid day of week (must be 0-6, Sunday=0)")]
    InvalidWeekday,

    #[msg("Invalid day of month (must be 1-28)")]
    InvalidMonthDay,

    #[msg("Custom interval must be positive")]
    InvalidCustomInterval,
}