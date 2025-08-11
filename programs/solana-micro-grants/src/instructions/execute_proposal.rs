use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer, TokenAccount};

use crate::state::{Proposal, ProposalStatus, Treasury};

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,

    #[account(
        mut,
        has_one = treasury,
        constraint = proposal.status == ProposalStatus::Approved as u8 @ ErrorCode::ProposalNotApproved
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        mut,
        seeds = [b"treasury", treasury.dao_name.as_bytes()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    /// CHECK: Validated in handler
    pub treasury_mint: UncheckedAccount<'info>,

    /// CHECK: Will be deserialized in handler
    #[account(mut)]
    pub treasury_token_account: UncheckedAccount<'info>,

    /// CHECK: Will be deserialized in handler
    #[account(mut)]
    pub recipient_token_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ExecuteProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;

    // Validate treasury mint matches proposal
    require!(
        ctx.accounts.treasury_mint.key() == proposal.treasury_mint,
        ErrorCode::MintMismatch
    );

    // Deserialize treasury token account
    let treasury_token_data = TokenAccount::try_deserialize(
        &mut &**ctx.accounts.treasury_token_account.try_borrow_data()?
    )?;
    require!(
        treasury_token_data.owner == ctx.accounts.treasury.key(),
        ErrorCode::InvalidTreasuryAccount
    );
    require!(
        treasury_token_data.mint == ctx.accounts.treasury_mint.key(),
        ErrorCode::TreasuryMintMismatch
    );

    // Deserialize recipient token account
    let recipient_token_data = TokenAccount::try_deserialize(
        &mut &**ctx.accounts.recipient_token_account.try_borrow_data()?
    )?;
    require!(
        recipient_token_data.mint == ctx.accounts.treasury_mint.key(),
        ErrorCode::RecipientMintMismatch
    );
    require!(
        ctx.accounts.recipient_token_account.key() == proposal.recipient,
        ErrorCode::InvalidRecipient
    );

    // Check voting deadline
    require!(
        clock.unix_timestamp >= proposal.voting_deadline,
        ErrorCode::VotingStillActive
    );

    // Check if passed
    require!(
        proposal.votes_for > proposal.votes_against,
        ErrorCode::ProposalDidNotPass
    );

    // Ensure treasury has balance
    require!(
        treasury_token_data.amount >= proposal.amount,
        ErrorCode::InsufficientTreasuryBalance
    );

    let transfer_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.treasury.to_account_info(),
    };

    let treasury_seeds: &[&[u8]] = &[
        b"treasury",
        ctx.accounts.treasury.dao_name.as_bytes(),
        &[ctx.accounts.treasury.bump],
    ];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            &[treasury_seeds],
        ),
        proposal.amount,
    )?;

    proposal.status = ProposalStatus::Executed as u8;

    msg!(
        "Proposal executed: {} tokens transferred to {}",
        proposal.amount,
        ctx.accounts.recipient_token_account.key()
    );

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Proposal has not been approved for execution.")]
    ProposalNotApproved,
    #[msg("Treasury token account mismatch.")]
    InvalidTreasuryAccount,
    #[msg("Token mint mismatch between treasury and proposal.")]
    MintMismatch,
    #[msg("Treasury token account mint mismatch.")]
    TreasuryMintMismatch,
    #[msg("Recipient token account mint mismatch.")]
    RecipientMintMismatch,
    #[msg("Invalid recipient account.")]
    InvalidRecipient,
    #[msg("Treasury has insufficient balance for this transfer.")]
    InsufficientTreasuryBalance,
    #[msg("Voting period is still active.")]
    VotingStillActive,
    #[msg("Proposal did not pass (more votes against than for).")]
    ProposalDidNotPass,
}
