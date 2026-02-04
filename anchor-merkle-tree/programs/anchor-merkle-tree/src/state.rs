use anchor_lang::prelude::*;

/// Lottery account that stores Merkle root and VRF result for weighted selection
#[account]
#[derive(InitSpace)]
pub struct Lottery {
    /// Authority that can update the lottery
    pub authority: Pubkey,
    /// Merkle root of the weighted holder tree
    pub merkle_root: [u8; 32],
    /// VRF result from Switchboard (32 bytes of randomness)
    pub vrf_result: [u8; 32],
    /// Total weight (sum of all balances) for modulo operation
    pub total_weight: u128,
    /// Winner address (set after claim_winner succeeds)
    pub winner: Option<Pubkey>,
    /// Winner's balance/weight
    pub winner_weight: u64,
    /// Whether the lottery has been finalized (winner claimed)
    pub finalized: bool,
    /// Bump seed for PDA
    pub bump: u8,
}

impl Lottery {
    pub const SEED_PREFIX: &'static [u8] = b"lottery";
}

/// Proof data for claiming winner status
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WinnerProof {
    /// The claimant's address (must match the signer or be verified)
    pub address: Pubkey,
    /// The claimant's balance/weight
    pub balance: u64,
    /// Cumulative weight before this entry (exclusive lower bound)
    pub prev_cumulative: u128,
    /// Cumulative weight including this entry (exclusive upper bound)
    pub cumulative: u128,
    /// Merkle proof hashes (path from leaf to root)
    pub proof: Vec<[u8; 32]>,
}
