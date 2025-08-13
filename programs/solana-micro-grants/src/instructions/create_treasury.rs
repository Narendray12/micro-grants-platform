use anchor_lang::prelude::*;
use crate::state::{ proposal::*};

#[derive(Accounts)]
#[instruction(dao_name: String)]
pub struct CreateTreasury<'info> {
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

pub fn create_treasury(
    ctx: Context<CreateTreasury>,
    dao_name: String
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;

    require!(
        dao_name.len() <= 32,
        ErrorCode::DaoNameTooLong
    );
    let mut name_bytes = [0u8; 32];
    let name_slice = dao_name.as_bytes();
    let copy_len = std::cmp::min(name_slice.len(), 32);
    name_bytes[..copy_len].copy_from_slice(&name_slice[..copy_len]);
    
    treasury.dao_name = name_bytes;
    treasury.bump = ctx.bumps.treasury;
    treasury.treasury_mint = treasury.key();

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("DAO name is too long.")]
    DaoNameTooLong,
    #[msg("Invalid treasury account provided.")]
    InvalidTreasuryAccount,
}