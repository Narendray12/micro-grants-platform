// In your state/dao.rs file:

use anchor_lang::prelude::*;

#[account]
pub struct DaoAccount {
    /// Authority that can manage the DAO
    pub authority: Pubkey,
    /// Name of the DAO
    pub dao_name: String,
    /// Treasury PDA that holds the funds
    pub treasury: Pubkey,
    /// Governance token mint (for voting power)
    pub treasury_mint: Pubkey,
    /// Total number of proposals created
    pub proposal_count: u64,
    /// PDA bump seed
    pub bump: u8,
}

impl DaoAccount {
    pub const MAX_DAO_NAME_LENGTH: usize = 50;

    pub const SPACE: usize = 8 + // Discriminator
        32 + // authority
        4 + Self::MAX_DAO_NAME_LENGTH + // dao_name (String with length prefix)
        32 + // treasury
        32 + // governance_token_mint
        8 +  // proposal_count
        1 +  // bump
        32; // padding for future fields
}
