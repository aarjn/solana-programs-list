use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod merkle;
pub mod state;

use instructions::*;
use state::WinnerProof;

declare_id!("HSBQg9YYMu8DtD1pgTfwxTqTdGWTKHtxSjg5wT3bz1mi");

#[program]
pub mod anchor_merkle_tree {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from anchor_merkle_tree!");
        Ok(())
    }

    pub fn commit_randomness(ctx: Context<CommitRandomness>) -> Result<()> {
        ctx.accounts.commit_randomness()
    }

    pub fn reveal_and_log_random(ctx: Context<RevealAndLog>) -> Result<()> {
        ctx.accounts.reveal_and_log_random()
    }

    /// Initialize a new lottery with a Merkle root and total weight
    pub fn initialize_lottery(
        ctx: Context<InitializeLottery>,
        merkle_root: [u8; 32],
        total_weight: u128,
    ) -> Result<()> {
        ctx.accounts
            .initialize_lottery(merkle_root, total_weight, &ctx.bumps)
    }

    /// Set the VRF result from a Switchboard randomness account
    pub fn set_vrf_result(ctx: Context<SetVrfResult>) -> Result<()> {
        ctx.accounts.set_vrf_result()
    }

    /// Claim winner status by providing a valid Merkle proof
    pub fn claim_winner(ctx: Context<ClaimWinner>, proof: WinnerProof) -> Result<()> {
        ctx.accounts.claim_winner(proof)
    }

    /// Close lottery and return rent to authority
    pub fn close_lottery(ctx: Context<CloseLottery>) -> Result<()> {
        ctx.accounts.close_lottery()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
