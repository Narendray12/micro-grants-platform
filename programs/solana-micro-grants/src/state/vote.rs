use anchor_lang::prelude::*;

#[account]
pub struct VoteAccount {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub choice: bool, // true = for, false = against
}
