// Implementacja logiki harmonogramów wypłat
//
// Główne funkcjonalności:
//
// 1. calculate_next_payout() - oblicza następną datę wypłaty
//    Parametry:
//    - schedule: PayoutSchedule - wybrany harmonogram
//    - current_timestamp: i64 - aktualny czas UNIX
//    
//    Obsługiwane harmonogramy:
//    - Disabled: zwraca 0 (brak wypłat)
//    - Daily: dodaje 86400 sekund (24h)
//    - Weekly: oblicza dzień tygodnia i dolicza odpowiednią liczbę dni
//    - Monthly: uproszczona implementacja (30 dni) + wybrany dzień miesiąca
//    - Custom: dodaje podany interwał w sekundach
//
// 2. should_payout() - decyduje czy wypłata powinna być wykonana
//    Warunki konieczne:
//    - auto_claim_enabled == true
//    - payout_schedule != Disabled
//    - current_dividend >= min_dividend_amount
//    - current_timestamp >= next_payout_date
//
// 3. validate_schedule() - waliduje poprawność ustawień harmonogramu
//    Sprawdza:
//    - Dni tygodnia (0-6 dla Weekly)
//    - Dni miesiąca (1-28 dla Monthly)
//    - Dodatni interwał dla Custom
//
// Obsługa błędów:
// - InvalidWeekday - niepoprawny dzień tygodnia
// - InvalidMonthDay - niepoprawny dzień miesiąca  
// - InvalidCustomInterval - niepoprawny interwał
//
// Uwagi:
// - Implementacja Monthly jest uproszczona (stałe 30 dni)
// - Obliczenia oparte na timestampach UNIX (sekundy od 1970)
// - Wszystkie funkcje są metodami statycznymi struktury ScheduleCalculator


use anchor_lang::prelude::*;
use crate::state::{UserStake, PayoutSchedule};
use crate::errors::ErrorCode;
pub struct ScheduleCalculator;

impl ScheduleCalculator {
    /// Oblicza następną datę wypłaty na podstawie harmonogramu i aktualnego czasu
    pub fn calculate_next_payout(
        schedule: PayoutSchedule,
        current_timestamp: i64
    ) -> Result<i64> {
        match schedule {
            PayoutSchedule::Disabled => Ok(0),
            PayoutSchedule::Daily => Ok(current_timestamp + 86400), // 24 godziny
            
            PayoutSchedule::Weekly(weekday) => {
                // Oblicz dzień tygodnia (0 = Niedziela, 6 = Sobota)
                let days_since_epoch = current_timestamp / 86400;
                let current_weekday = (days_since_epoch + 4) % 7; // Dostosowanie do timestampów
                
                // Oblicz dni do następnego wybranego dnia tygodnia
                let days_to_add = if weekday as i64 > current_weekday {
                    weekday as i64 - current_weekday
                } else {
                    7 - (current_weekday - weekday as i64)
                };
                
                Ok(current_timestamp + days_to_add * 86400)
            },
            
            PayoutSchedule::Monthly(day_of_month) => {
                // Uproszczona implementacja - zawsze 30 dni
                let next_month_timestamp = current_timestamp + 30 * 86400;
                // Ustawiamy na wybrany dzień miesiąca (1-28)
                Ok(next_month_timestamp - (next_month_timestamp % 86400) + ((day_of_month as i64 - 1) * 86400))
            },
            
            PayoutSchedule::Custom(interval_secs) => {
                Ok(current_timestamp + interval_secs)
            }
        }
    }

    /// Sprawdza czy wypłata powinna zostać wykonana
    pub fn should_payout(
        user_stake: &UserStake,
        current_dividend: u64,
        current_timestamp: i64
    ) -> Result<bool> {
        // Warunki blokujące automatyczną wypłatę
        if !user_stake.auto_claim_enabled 
            || user_stake.payout_schedule == PayoutSchedule::Disabled
            || current_dividend < user_stake.min_dividend_amount {
            return Ok(false);
        }

        // Sprawdź czy osiągnięto czas wypłaty
        Ok(current_timestamp >= user_stake.next_payout_date)
    }

    /// Weryfikuje poprawność ustawień harmonogramu
    pub fn validate_schedule(
        schedule: &PayoutSchedule
    ) -> Result<()> {
        match schedule {
            PayoutSchedule::Weekly(day) if *day > 6 => {
                Err(ErrorCode::InvalidWeekday.into())
            },
            PayoutSchedule::Monthly(day) if *day < 1 || *day > 28 => {
                Err(ErrorCode::InvalidMonthDay.into())
            },
            PayoutSchedule::Custom(secs) if *secs <= 0 => {
                Err(ErrorCode::InvalidCustomInterval.into())
            },
            _ => Ok(())
        }
    }
}