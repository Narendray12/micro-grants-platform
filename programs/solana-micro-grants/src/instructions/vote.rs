use crate::state::{proposal::*, vote::*};
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init,
        payer = voter,
        space = 8 + 32 + 32 + 1 + 32,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, VoteAccount>,

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Vote>, choice: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let vote = &mut ctx.accounts.vote;

    require!(
        proposal.status == ProposalStatus::Active as u8,
        ErrorCode::ProposalNotActive
    );

    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp <= proposal.voting_deadline,
        ErrorCode::VotingClosed
    );

    vote.proposal = proposal.key();
    vote.voter = ctx.accounts.voter.key();
    vote.choice = choice;

    if choice {
        proposal.votes_for += 1;
    } else {
        proposal.votes_against += 1;
    }

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Proposal is not active.")]
    ProposalNotActive,
    #[msg("Voting period is over.")]
    VotingClosed,
}