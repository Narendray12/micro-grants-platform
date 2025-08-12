use anchor_lang::prelude::*;
use crate::state::{dao::*, proposal::*};

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut, has_one = authority)]
    pub dao: Account<'info, DaoAccount>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 64 + 256 + 8 + 32 + 8 + 8 + 1 + 8 + 1,
        seeds = [b"proposal", dao.key().as_ref(), &dao.proposal_count.to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}


pub fn create_progosal(
    ctx: Context<CreateProposal>,
    title: String,
    description: String,
    amount: u64,
    recipient: Pubkey,
    voting_deadline: i64
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let dao = &mut ctx.accounts.dao;
    
    let mut title_bytes = [0u8; 64];
    let title_slice = title.as_bytes();
    let copy_len = std::cmp::min(title_slice.len(), 64);
    title_bytes[..copy_len].copy_from_slice(&title_slice[..copy_len]);

    let mut description_bytes = [0u8; 256];
    let description_slice = description.as_bytes();
    let copy_len = std::cmp::min(description_slice.len(), 256);
    description_bytes[..copy_len].copy_from_slice(&description_slice[..copy_len]);

    proposal.key = dao.key();
    proposal.title = title_bytes;
    proposal.description = description_bytes;
    proposal.recipient = recipient;
    proposal.votes_for = 0;
    proposal.votes_against = 0;
    proposal.status = ProposalStatus::Active as u8;
    proposal.voting_deadline = voting_deadline;
    proposal.treasury = dao.treasury;
    proposal.amount = amount;
    proposal.bump = ctx.bumps.proposal;

    dao.proposal_count += 1;

    Ok(())
}
