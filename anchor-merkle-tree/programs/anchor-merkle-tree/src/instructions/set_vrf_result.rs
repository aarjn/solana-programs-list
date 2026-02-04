use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::errors::ErrorCode;
use crate::state::Lottery;

#[derive(Accounts)]
pub struct SetVrfResult<'info> {
    #[account(
        mut,
        seeds = [Lottery::SEED_PREFIX, authority.key().as_ref()],
        bump = lottery.bump,
        has_one = authority,
        constraint = !lottery.finalized @ ErrorCode::LotteryAlreadyFinalized
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK: Validated by Switchboard SDK
    pub randomness_account: AccountInfo<'info>,

    pub authority: Signer<'info>,
}

impl<'info> SetVrfResult<'info> {
    pub fn set_vrf_result(&mut self) -> Result<()> {
        // Parse the Switchboard randomness account
        let randomness_data =
            RandomnessAccountData::parse(self.randomness_account.data.borrow()).map_err(|_| {
                msg!("Failed to parse randomness account data");
                ErrorCode::InvalidRandomnessData
            })?;

        // Ensure randomness has been revealed
        if randomness_data.reveal_slot == 0 {
            return Err(ErrorCode::RandomnessNotRevealed.into());
        }

        // Get the current clock
        let clock = Clock::get().map_err(|_| ErrorCode::ClockError)?;

        // Get the random value (32 bytes)
        let vrf_result = randomness_data.get_value(clock.slot).map_err(|e| {
            msg!("Failed to get randomness value: {:?}", e);
            ErrorCode::RandomnessNotResolved
        })?;

        // Store the VRF result
        self.lottery.vrf_result = vrf_result;

        msg!("VRF result set successfully");
        msg!(
            "First 8 bytes as u64: {}",
            u64::from_le_bytes(vrf_result[..8].try_into().unwrap())
        );

        Ok(())
    }
}
