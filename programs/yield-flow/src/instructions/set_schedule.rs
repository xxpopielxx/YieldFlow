use anchor_lang::prelude::*;
use crate::{state::UserStake, utils::ScheduleCalculator};

#[derive(Accounts)]
pub struct SetSchedule<'info> {
    #[account(mut, has_one = user)]
    pub user_stake: Account<'info, UserStake>,
    pub user: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetScheduleParams {
    pub schedule: PayoutSchedule,
    pub auto_claim: bool,
    pub min_amount: u64,
}

pub fn handler(ctx: Context<SetSchedule>, params: SetScheduleParams) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    
    user_stake.payout_schedule = params.schedule;
    user_stake.auto_claim_enabled = params.auto_claim;
    user_stake.min_dividend_amount = params.min_amount;
    
    // Oblicz nową datę wypłaty
    user_stake.next_payout_date = ScheduleCalculator::calculate_next_payout(
        params.schedule,
        Clock::get()?.unix_timestamp
    )?;
    
    Ok(())
}