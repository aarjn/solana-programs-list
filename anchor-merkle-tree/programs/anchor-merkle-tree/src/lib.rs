use anchor_lang::prelude::*;

declare_id!("HSBQg9YYMu8DtD1pgTfwxTqTdGWTKHtxSjg5wT3bz1mi");

#[program]
pub mod anchor_merkle_tree {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
