use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct CommitRandomness<'info> {
    /// CHECK: The randomness account data is validated manually within the handler
    pub randomness_account: AccountInfo<'info>,
    pub user: Signer<'info>,
}

impl<'info> CommitRandomness<'info> {
    pub fn commit_randomness(&mut self) -> Result<()> {
        // Get the current clock
        let clock = anchor_lang::prelude::Clock::get().map_err(|_| ErrorCode::ClockError)?;

        // Parse the randomness account data to verify it's valid
        let randomness_data = RandomnessAccountData::parse(self.randomness_account.data.borrow())
            .map_err(|_| ErrorCode::InvalidRandomnessData)?;

        msg!("Randomness committed successfully!");
        msg!("Seed slot: {}", randomness_data.seed_slot);
        msg!("Current slot: {}", clock.slot);
        msg!("Randomness account: {}", self.randomness_account.key());

        Ok(())
    }
}
