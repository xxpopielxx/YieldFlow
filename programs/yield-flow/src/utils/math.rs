// Moduł obliczeń finansowych 
//
// Zawiera dwie główne funkcje:
//
// 1. calculate_dividend() - oblicza wartość dywidendy w lamportach SOL
//    Parametry:
//    - msol_amount: u64 - ilość tokenów mSOL (1 mSOL = 1_000_000_000 lamportów)
//    - base_sol_value: u64 - początkowa wartość 1 mSOL w lamportach SOL
//    - current_sol_value: u64 - aktualna wartość 1 mSOL w lamportach SOL
//    
//    Proces:
//    1. Sprawdza czy aktualna wartość jest większa od początkowej
//    2. Oblicza różnicę wartości z zabezpieczeniem przed overflow
//    3. Mnoży ilość mSOL przez różnicę wartości
//    4. Konwertuje wynik z precyzji mSOL do SOL (dzieli przez 1_000_000_000)
//
//    Zwraca:
//    - Ilość lamportów SOL należnych jako dywidenda
//    - Błąd jeśli dywidenda jest zbyt mała lub wystąpi overflow
//
// 2. calculate_compound_interest() - oblicza procent składany
//    Parametry:
//    - principal: u64 - kapitał początkowy w lamportach
//    - rate_per_period: u64 - stopa procentowa w punktach bazowych (1% = 100)
//    - periods: u64 - liczba okresów kapitalizacji
//
//    Proces:
//    1. Sprawdza przypadki brzegowe (stopa lub okresy = 0)
//    2. Wykonuje iteracyjne obliczenia procentu składanego
//    3. Dla każdego okresu:
//       - Mnoży przez czynnik (10_000 + rate)
//       - Dzieli przez 10_000 (zabezpieczenie przed ułamkami)
//
//    Zwraca:
//    - Sumę odsetek w lamportach
//    - Błąd jeśli wystąpi overflow
//
// Stałe:
// - 1_000_000_000 - precyzja konwersji mSOL do SOL
// - 10_000 - podstawa dla punktów bazowych (1% = 100)
//
// Obsługa błędów:
// - DividendTooSmall - gdy aktualna wartość ≤ początkowej
// - MathOverflow - gdy operacja matematyczna przekracza zakres
//
// Uwagi:
// - Wszystkie obliczenia wykonują sprawdzanie overflow
// - Działa na wartościach w lamportach (1 SOL = 1_000_000_000 lamportów)
// - Zoptymalizowane pod kątem bezpieczeństwa i dokładności


use anchor_lang::prelude::*;
use crate::errors::ErrorCode;


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
    
    // Convert from mSOL precision to SOL (1 mSOL = 1e9 lamports)
    raw_dividend.checked_div(1_000_000_000)
        .ok_or(ErrorCode::MathOverflow.into())
}


pub fn calculate_compound_interest(
    principal: u64,
    rate_per_period: u64,
    periods: u64
) -> Result<u64> {
    if rate_per_period == 0 || periods == 0 {
        return Ok(0);
    }

    let rate_factor = 10_000u128;
    let mut amount = principal as u128;
    
    for _ in 0..periods {
        amount = amount
            .checked_mul(rate_factor + rate_per_period as u128)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(rate_factor)
            .ok_or(ErrorCode::MathOverflow)?;
    }

    Ok((amount - principal as u128) as u64)
}