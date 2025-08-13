use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
use instructions::*;
declare_id!("B3soWfYRTR2R4J9oDdky11nz5m1k92sizz9TcW2jaGps");

#[program]
pub mod Solana_Micro_Grants {

    use super::*;

    pub fn create_dao(ctx: Context<CreateDao>, dao_name: String) -> Result<()> {
        create_dao::create_dao(ctx, dao_name)
    }
    pub fn create_treasury(ctx: Context<CreateTreasury>, dao_name: String) -> Result<()> {
        create_treasury::create_treasury(ctx, dao_name)
    }

    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        deposit_funds::deposit_funds(ctx, amount)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        cid: String,
        amount: u64,
        recipient: Pubkey,
        voting_deadline: i64,
    ) -> Result<()> {
        create_proposal::create_progosal(ctx, cid, amount, recipient, voting_deadline)
    }

    pub fn vote(ctx: Context<Vote>, choice: bool) -> Result<()> {
        vote::handler(ctx, choice)
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        execute_proposal::execute_proposal(ctx);
        Ok(())
    }
}
