use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("HSBQg9YYMu8DtD1pgTfwxTqTdGWTKHtxSjg5wT3bz1mi");

#[program]
pub mod anchor_merkle_tree {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn commit_randomness(ctx: Context<CommitRandomness>) -> Result<()> {
        ctx.accounts.commit_randomness()
    }

    pub fn reveal_and_log_random(ctx: Context<RevealAndLog>) -> Result<()> {
        ctx.accounts.reveal_and_log_random()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
