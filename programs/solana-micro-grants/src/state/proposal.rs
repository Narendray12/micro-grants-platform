use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Active = 0,
    Passed = 1,
    Rejected = 2,
    Approved = 3,
    Executed = 4,
}

#[account]
pub struct Treasury {
    pub dao_name: String,
    pub balance: u64,
    pub bump: u8,
}

impl Treasury {
    pub const MAX_DAO_NAME_LENGTH: usize = 50;

    pub const SPACE: usize = 8 + // Discriminator
        4 + Self::MAX_DAO_NAME_LENGTH + // dao_name (String with length prefix)
        8 + // balance
        1;// bump
}

#[account]
pub struct Proposal {
    pub key: Pubkey,
    pub dao_name: String,
    pub title: String,
    pub description: String,
    pub amount: u64,       // in lamports or smallest SPL unit
    pub recipient: Pubkey, // where funds go if approved
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: u8,           // ProposalStatus as u8
    pub voting_deadline: i64, // Unix timestamp
    pub bump: u8,
    pub treasury_mint: Pubkey, // Token mint for the transfer
    pub treasury: Pubkey,      // Treasury PDA this proposal belongs to
}

impl Proposal {
    pub const MAX_DAO_NAME_LENGTH: usize = 50;
    pub const MAX_TITLE_LENGTH: usize = 100;
    pub const MAX_DESCRIPTION_LENGTH: usize = 500;

    pub const SPACE: usize = 8 + // Discriminator
        4 + Self::MAX_DAO_NAME_LENGTH + // dao_name
        4 + Self::MAX_TITLE_LENGTH +    // title
        4 + Self::MAX_DESCRIPTION_LENGTH + // description
        8 +  // amount
        32 + // recipient
        8 +  // votes_for
        8 +  // votes_against
        1 +  // status
        8 +  // voting_deadline
        1 +  // bump
        32 + // treasury_mint
        32 + // treasury
        32; // padding

    /// Check if voting period is still active
    pub fn is_voting_active(&self) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        current_time < self.voting_deadline && self.status == ProposalStatus::Active as u8
    }

    /// Check if proposal passed the vote
    pub fn has_passed(&self) -> bool {
        self.votes_for > self.votes_against
    }

    /// Check if proposal can be executed
    pub fn can_execute(&self) -> bool {
        self.status == ProposalStatus::Approved as u8
    }

    /// Get the current status as enum
    pub fn get_status(&self) -> ProposalStatus {
        match self.status {
            0 => ProposalStatus::Active,
            1 => ProposalStatus::Passed,
            2 => ProposalStatus::Rejected,
            3 => ProposalStatus::Approved,
            4 => ProposalStatus::Executed,
            _ => ProposalStatus::Rejected, // Default fallback
        }
    }
}
