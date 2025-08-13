use anchor_lang::prelude::*;
use crate::state::{dao::*};

#[derive(Accounts)]
#[instruction(dao_name: String)]
pub struct CreateDao<'info> {
    #[account(
        init, 
        payer = authority, 
        space = DaoAccount::SPACE,
        seeds = [b"dao", dao_name.as_bytes()],
        bump
    )]
    pub dao: Account<'info, DaoAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_dao(
    ctx: Context<CreateDao>,
    dao_name: String
) -> Result<()> {
    let dao = &mut ctx.accounts.dao;

    require!(
        dao_name.len() <= 32,
        ErrorCode::DaoNameTooLong
    );
    let mut name_bytes = [0u8; 32];
    let name_slice = dao_name.as_bytes();
    let copy_len = std::cmp::min(name_slice.len(), 32);
    name_bytes[..copy_len].copy_from_slice(&name_slice[..copy_len]);
    
    dao.authority = ctx.accounts.authority.key();
    dao.dao_name = name_bytes;
    dao.proposal_count = 0;
    dao.bump = ctx.bumps.dao;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("DAO name is too long.")]
    DaoNameTooLong,
    #[msg("Invalid treasury account provided.")]
    InvalidTreasuryAccount,
}
