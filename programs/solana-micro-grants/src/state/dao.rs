use anchor_lang::prelude::*;

#[account]
pub struct DaoAccount {
    /// Authority that can manage the DAO
    pub authority: Pubkey,
    /// Treasury PDA that holds the funds  
    pub proposal_count: u64,
    /// PDA bump seed
    pub bump: u8,
    /// Name of the DAO (shortened and at the end)
    pub dao_name: [u8; 32], // Fixed-size array instead of String
}

impl DaoAccount {
    pub const MAX_NAME_LEN: usize = 32;
    pub const SPACE: usize = 8 + 
        32 + 
        32 + 
        1 +  
        32;  
}
