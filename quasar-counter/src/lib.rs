#![cfg_attr(not(test), no_std)]

pub mod instructions;
pub use instructions::*;
use quasar_lang::prelude::*;

declare_id!("9QWoDrTfMpWpgeh8qP9FqY6NdyqordB6qbfndEw6HjxY");

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub payer: &'info mut Signer,
    pub system_program: &'info Program<System>,
}

impl<'info> Initialize<'info> {
    #[inline(always)]
    pub fn initialize(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}

#[program]
mod quasar_counter {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        ctx.accounts.initialize()
    }
}

#[cfg(test)]
mod tests;
