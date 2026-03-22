#![cfg_attr(not(test), no_std)]

pub mod instructions;
pub mod state;

pub use instructions::*;
use quasar_lang::prelude::*;

declare_id!("49BmG32poqLgu6RSajXSBUYxPsp9JZhvNCgmVwDzeRWb");

#[program]
mod quasar_escrow {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn make(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        ctx.accounts.make()
    }
}

#[cfg(test)]
mod tests;
