use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid randomness data")]
    InvalidRandomnessData,
    #[msg("Randomness not yet resolved")]
    RandomnessNotResolved,
    #[msg("Randomness not yet revealed")]
    RandomnessNotRevealed,
    #[msg("Clock error")]
    ClockError,
}
