use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Proposal is not active.")]
    ProposalNotActive,
    #[msg("Voting period is over.")]
    VotingClosed,
    #[msg("Proposal already executed.")]
    AlreadyExecuted,
    #[msg("Proposal did not pass.")]
    ProposalRejected,
}
