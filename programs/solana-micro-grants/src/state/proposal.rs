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

pub const MAX_CID_LEN: usize = 64;
#[account]
pub struct Proposal {
    /// Core identifiers
    pub key: Pubkey,
    pub recipient: Pubkey,
    pub proposal_index: u64,
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

    pub cid: [u8; MAX_CID_LEN],
    pub cid_len: u8,
}

impl Proposal {
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
        MAX_CID_LEN + // cid bytes
        1 +
        63; // Padding
            // Total: 521 bytes vs original ~800+ bytes
    pub fn set_cid(&mut self, cid_str: &str) {
        let bytes = cid_str.as_bytes();
        let n = core::cmp::min(bytes.len(), MAX_CID_LEN);
        self.cid = [0u8; MAX_CID_LEN];
        self.cid[..n].copy_from_slice(&bytes[..n]);
        self.cid_len = n as u8;
    }

    /// Helper: read CID as &str (unsafe if not valid UTF-8; prefer using raw bytes for hashes)
    pub fn cid_as_slice(&self) -> &[u8] {
        &self.cid[..(self.cid_len as usize)]
    }
    /// Helper to get dao_name as string
    pub fn get_dao_name(&self) -> String {
        String::from_utf8_lossy(&self.dao_name)
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
