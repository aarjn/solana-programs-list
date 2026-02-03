use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct RevealAndLog<'info> {
    /// CHECK: The randomness account data is validated manually within the handler
    pub randomness_account: AccountInfo<'info>,
    pub user: Signer<'info>,
}

impl<'info> RevealAndLog<'info> {
    pub fn reveal_and_log_random(&mut self) -> Result<()> {
        // Get the current clock for randomness validation
        let clock: Clock = Clock::get()?;

        // Parse the randomness account data
        let randomness_data = RandomnessAccountData::parse(self.randomness_account.data.borrow())
            .map_err(|_| ErrorCode::InvalidRandomnessData)?;

        // Check if randomness has been revealed
        if randomness_data.reveal_slot == 0 {
            msg!("Reveal slot is 0 - randomness not yet revealed");
            return Err(ErrorCode::RandomnessNotRevealed.into());
        }

        msg!(
            "Randomness revealed at slot: {}",
            randomness_data.reveal_slot
        );

        // Get the revealed random value
        let random_bytes = randomness_data
            .get_value(clock.slot)
            .map_err(|_| ErrorCode::RandomnessNotResolved)?;

        // Convert first 8 bytes to u64 for logging
        let random_u64 = u64::from_le_bytes([
            random_bytes[0],
            random_bytes[1],
            random_bytes[2],
            random_bytes[3],
            random_bytes[4],
            random_bytes[5],
            random_bytes[6],
            random_bytes[7],
        ]);

        // Log the random value
        msg!("🎲 Random value generated: {}", random_u64);
        msg!("🔢 Random bytes: {:?}", &random_bytes[0..8]);
        msg!("📅 Seed slot: {}", randomness_data.seed_slot);
        msg!("🔓 Reveal slot: {}", randomness_data.reveal_slot);
        msg!("⏰ Current slot: {}", clock.slot);

        Ok(())
    }
}
