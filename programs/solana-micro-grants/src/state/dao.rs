use anchor_lang::prelude::*;

#[account]
pub struct DaoAccount {
    pub authority: Pubkey,
    pub proposal_count: u64,
    pub bump: u8,
    pub dao_name: [u8; 32], 
}

impl DaoAccount {
    pub const MAX_NAME_LEN: usize = 32;
    pub const SPACE: usize = 8 + 
        32 + 
        32 + 
        1 +  
        32+
        32;
}
