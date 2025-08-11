use anchor_lang::prelude::*;
use crate::state::dao::*;
#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    pub dao: Account<'info, DaoAccount>,

    #[account(mut)]
    pub from: Signer<'info>,

    #[account(mut)]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.from;
    let treasury = &mut ctx.accounts.treasury;

    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **treasury.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}
