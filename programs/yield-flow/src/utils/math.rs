use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

/// Calculates the dividend amount based on mSOL amount and value difference
pub fn calculate_dividend(
    msol_amount: u64,
    base_sol_value: u64,
    current_sol_value: u64
) -> Result<u64> {
    // Verify current value is higher than base value
    if current_sol_value <= base_sol_value {
        return Err(ErrorCode::DividendTooSmall.into());
    }

    // Calculate value difference with overflow check
    let value_diff = current_sol_value.checked_sub(base_sol_value)
        .ok_or(ErrorCode::MathOverflow)?;
    
    // Calculate raw dividend amount (mSOL * value difference)
    let raw_dividend = msol_amount.checked_mul(value_diff)
        .ok_or(ErrorCode::MathOverflow)?;
    
    // Convert to SOL (divide by 1_000_000 lamports per SOL)
    raw_dividend.checked_div(1_000_000)
        .ok_or(ErrorCode::MathOverflow)
}

/// Calculates compound interest for staking period
pub fn calculate_compound_interest(
    principal: u64,
    rate_per_period: u64, // in basis points (1% = 100)
    periods: u64
) -> Result<u64> {
    if rate_per_period == 0 || periods == 0 {
        return Ok(principal);
    }

    let mut amount = principal;
    for _ in 0..periods {
        let interest = amount.checked_mul(rate_per_period)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(10_000) // 10000 basis points = 100%
            .ok_or(ErrorCode::MathOverflow)?;
        
        amount = amount.checked_add(interest)
            .ok_or(ErrorCode::MathOverflow)?;
    }

    Ok(amount - principal)
}

/// Calculates fee amount based on principal and fee rate
pub fn calculate_fee(
    amount: u64,
    fee_rate_bps: u16 // fee in basis points (1% = 100)
) -> Result<u64> {
    if fee_rate_bps == 0 {
        return Ok(0);
    }

    amount.checked_mul(fee_rate_bps as u64)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(10_000)
        .ok_or(ErrorCode::MathOverflow)
}

/// Safe percentage calculation (value * percentage / 100)
pub fn calculate_percentage(
    value: u64,
    percentage: u16
) -> Result<u64> {
    value.checked_mul(percentage as u64)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(100)
        .ok_or(ErrorCode::MathOverflow)
}


/// Helper function to calculate APR to daily rate
pub fn apr_to_daily_rate(apr_bps: u64) -> Result<u64> {
    // APR in basis points to daily rate (365 days)
    apr_bps.checked_div(365)
        .ok_or(ErrorCode::MathOverflow)
}