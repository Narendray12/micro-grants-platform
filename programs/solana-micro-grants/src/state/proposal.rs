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
    pub dao_name: [u8; 32],
    pub treasury_mint: Pubkey,
    pub bump: u8,
}

impl Treasury {
    pub const SPACE: usize = 8 + 32 + 32 + 1; // 49 bytes (removed unnecessary 4 bytes)
}

#[account]
pub struct Proposal {
    /// Core identifiers
    pub key: Pubkey,
    pub recipient: Pubkey,
    pub treasury_mint: Pubkey,
    pub treasury: Pubkey,

    /// Financial data
    pub amount: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub voting_deadline: i64,

    /// Packed data: status (3 bits) + bump (5 bits)
    pub status: u8,
    pub bump: u8,
    /// Fixed-size strings (no length prefixes)
    pub dao_name: [u8; 32], // Reduced from 50
    pub title: [u8; 64],        // Reduced from 100
    pub description: [u8; 256], // Reduced from 500
}

impl Proposal {
    pub const MAX_DAO_NAME_LENGTH: usize = 32;
    pub const MAX_TITLE_LENGTH: usize = 64;
    pub const MAX_DESCRIPTION_LENGTH: usize = 256;

    pub const SPACE: usize = 8 +   // Discriminator
        32 + // key
        32 + // recipient  
        32 + // treasury_mint
        32 + // treasury
        8 +  // amount
        8 +  // votes_for
        8 +  // votes_against  
        8 +  // voting_deadline
        1 +  // status
        1 +  // bump
        32 + // dao_name (fixed)
        64 + // title (fixed)
        256; // description (fixed)
             // Total: 521 bytes vs original ~800+ bytes

    /// Helper to get dao_name as string
    pub fn get_dao_name(&self) -> String {
        String::from_utf8_lossy(&self.dao_name)
            .trim_end_matches('\0')
            .to_string()
    }

    /// Helper to get title as string
    pub fn get_title(&self) -> String {
        String::from_utf8_lossy(&self.title)
            .trim_end_matches('\0')
            .to_string()
    }

    /// Helper to get description as string
    pub fn get_description(&self) -> String {
        String::from_utf8_lossy(&self.description)
            .trim_end_matches('\0')
            .to_string()
    }

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
        self.status == ProposalStatus::Approved as u8 && self.is_voting_active()
    }
}
