use anchor_lang::prelude::*;
use crate::state::{UserStake, PayoutSchedule};

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
                return Err(ErrorCode::InvalidScheduleConfig.into());
            },
            PayoutSchedule::Monthly(day) if *day < 1 || *day > 28 => {
                return Err(ErrorCode::InvalidScheduleConfig.into());
            },
            PayoutSchedule::Custom(secs) if *secs <= 0 => {
                return Err(ErrorCode::InvalidScheduleConfig.into());
            },
            _ => Ok(())
        }
    }
}