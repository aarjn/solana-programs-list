pub mod error;
pub mod hash;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("FvqTgpWEMR5ZT4jikL4XeQhX8qsghPWEyKKoP3Esdukn");

#[program]
pub mod merkle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn insert(ctx: Context<Insert>, leaf: [u8; 32]) -> Result<()> {
        instructions::insert::handler(ctx, leaf)
    }

    pub fn verify(ctx: Context<Verify>, leaf: [u8; 32], index: u32, proof: [[u8; 32]; 20]) -> Result<()> {
        instructions::verify::handler(ctx, leaf, index, proof)
    }
}
