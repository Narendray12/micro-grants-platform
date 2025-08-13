use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{dao::*, proposal::*};

#[derive(Accounts)]
#[instruction(dao_name: String, proposal_index: u64)]
pub struct ExecuteProposal<'info> {
    #[account(
        mut,
        seeds = [b"dao", dao_name.as_bytes()],
        bump = dao.bump
    )]
    pub dao: Account<'info, DaoAccount>,

    #[account(
        mut,
        seeds = [b"treasury", dao_name.as_bytes()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [b"proposal", dao.key().as_ref(), &proposal_index.to_le_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,

    /// Treasury's SPL token account holding DAO funds
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// Recipient's SPL token account
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub authority: Signer<'info>,
}

pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let treasury = &mut ctx.accounts.treasury;
    let proposal = &mut ctx.accounts.proposal;
    let _dao_name = proposal.get_dao_name();
    let _proposal_index = proposal.proposal_index;
    // Ensure proposal is approved
    require!(
        proposal.status == ProposalStatus::Approved as u8,
        ErrorCode::ProposalNotApproved
    );
    require!(
        proposal.key()
            == Pubkey::find_program_address(
                &[
                    b"proposal",
                    dao.key().as_ref(),
                    &_proposal_index.to_le_bytes()
                ],
                ctx.program_id
            )
            .0,
        ErrorCode::InvalidProposal
    );

    // Transfer tokens from treasury to recipient
    let seeds = &[b"treasury", &dao.dao_name as &[u8], &[treasury.bump]];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: treasury.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );

    token::transfer(cpi_ctx, proposal.amount)?;

    proposal.status = ProposalStatus::Executed as u8;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Proposal is not approved for execution.")]
    ProposalNotApproved,
    #[msg("Invalid proposal index.")]
    InvalidProposalIndex,
    #[msg("Invalid proposal account provided.")]
    InvalidProposal,
}
