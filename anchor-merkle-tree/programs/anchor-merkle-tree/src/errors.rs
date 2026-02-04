use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // Randomness errors
    #[msg("Invalid randomness data")]
    InvalidRandomnessData,
    #[msg("Randomness not yet resolved")]
    RandomnessNotResolved,
    #[msg("Randomness not yet revealed")]
    RandomnessNotRevealed,
    #[msg("Clock error")]
    ClockError,

    // Lottery state errors
    #[msg("Lottery has already been finalized")]
    LotteryAlreadyFinalized,
    #[msg("VRF result has not been set")]
    VrfNotSet,

    // Winner claim errors
    #[msg("Claimant does not match proof address")]
    InvalidClaimant,
    #[msg("Invalid cumulative weight range")]
    InvalidCumulativeRange,
    #[msg("Balance does not match cumulative range")]
    BalanceMismatch,
    #[msg("Cumulative weight exceeds total weight")]
    CumulativeExceedsTotalWeight,
    #[msg("Invalid Merkle proof")]
    InvalidMerkleProof,
    #[msg("Selection point does not fall within claimant's range - not the winner")]
    NotTheWinner,
}
