use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
use instructions::*;
use crate::state::DaoAccount;
declare_id!("B3soWfYRTR2R4J9oDdky11nz5m1k92sizz9TcW2jaGps");

#[program]
pub mod Solana_Micro_Grants {

    use super::*;

    pub fn create_dao(ctx: Context<CreateDao>, governance_token_mint: Pubkey, dao_name: String) -> Result<()> {
        create_dao::create_dao(ctx,  governance_token_mint,dao_name)
    }

    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        deposit_funds::deposit_funds(ctx, amount)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        amount: u64,
        recipient: Pubkey,
        voting_deadline: i64
    ) -> Result<()> {
        create_proposal::create_progosal(ctx, title, description, amount, recipient, voting_deadline)
    }

    pub fn vote(ctx: Context<Vote>, choice: bool) -> Result<()> {
        vote::handler(ctx, choice)
    }

      pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        execute_proposal::handler(ctx)
    }
}

