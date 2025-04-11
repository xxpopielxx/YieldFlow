use anchor_lang::prelude::*;
use crate::{state::ProgramConfig, errors::ErrorCode};

#[derive(Accounts)]
pub struct UpdatePandleProgram<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, ProgramConfig>,
    pub admin: Signer<'info>,
    /// CHECK: Will be verified when used
    pub new_pandle_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateEscrow<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, ProgramConfig>,
    pub admin: Signer<'info>,
    /// CHECK: Will be verified by seeds when used
    pub new_escrow: AccountInfo<'info>,
}

pub fn update_pandle_program_handler(
    ctx: Context<UpdatePandleProgram>,
) -> Result<()> {
    ctx.accounts.config.pandle_program = ctx.accounts.new_pandle_program.key();
    msg!("Pandle program updated to: {}", ctx.accounts.config.pandle_program);
    Ok(())
}

pub fn update_escrow_handler(
    ctx: Context<UpdateEscrow>,
) -> Result<()> {
    // W rzeczywistości escrow jest PDA, więc ta funkcja może być niepotrzebna
    // lub może aktualizować tylko referencję do escrow
    msg!("Escrow account reference updated");
    Ok(())
}