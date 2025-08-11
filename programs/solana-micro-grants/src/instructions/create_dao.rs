use anchor_lang::prelude::*;
use crate::state::{dao::*, Treasury};

#[derive(Accounts)]
#[instruction(dao_name: String, governance_token_mint: Pubkey)]
pub struct CreateDao<'info> {
    #[account(
        init, 
        payer = authority, 
        space = DaoAccount::SPACE,
        seeds = [b"dao", dao_name.as_bytes()],
        bump
    )]
    pub dao: Account<'info, DaoAccount>,

    #[account(
        init,
        payer = authority,
        space = Treasury::SPACE,
        seeds = [b"treasury", dao_name.as_bytes()],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_dao(
    ctx: Context<CreateDao>,
    dao_name: String,
    governance_token_mint: Pubkey
) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let treasury = &mut ctx.accounts.treasury;

    // Validate DAO name length
    require!(
        dao_name.len() <= DaoAccount::MAX_DAO_NAME_LENGTH,
        ErrorCode::DaoNameTooLong
    );

    // Initialize DAO account
    dao.authority = ctx.accounts.authority.key();
    dao.dao_name = dao_name.clone();
    dao.treasury = treasury.key();
    dao.treasury_mint = governance_token_mint;
    dao.proposal_count = 0;
    dao.bump = ctx.bumps.dao;

    // Initialize Treasury account
    treasury.dao_name = dao_name;
    treasury.bump = ctx.bumps.treasury;
    treasury.balance = 0; // or handle token accounts here

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("DAO name is too long.")]
    DaoNameTooLong,
    #[msg("Invalid treasury account provided.")]
    InvalidTreasuryAccount,
}
