use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::merkle::{compute_leaf_hash, compute_selection_point, is_winner, verify_proof};
use crate::state::{Lottery, WinnerProof};

#[derive(Accounts)]
pub struct ClaimWinner<'info> {
    #[account(
        mut,
        seeds = [Lottery::SEED_PREFIX, lottery.authority.as_ref()],
        bump = lottery.bump,
        constraint = !lottery.finalized @ ErrorCode::LotteryAlreadyFinalized,
        constraint = lottery.vrf_result != [0u8; 32] @ ErrorCode::VrfNotSet
    )]
    pub lottery: Account<'info, Lottery>,

    /// The claimant - must match the address in the proof
    pub claimant: Signer<'info>,
}

impl<'info> ClaimWinner<'info> {
    pub fn claim_winner(&mut self, proof: WinnerProof) -> Result<()> {
        let lottery = &mut self.lottery;

        // 1. Verify the claimant matches the proof address
        require!(
            proof.address == self.claimant.key(),
            ErrorCode::InvalidClaimant
        );

        // 2. Verify cumulative range is valid
        require!(
            proof.cumulative > proof.prev_cumulative,
            ErrorCode::InvalidCumulativeRange
        );

        // 3. Verify balance matches the range
        let expected_balance = proof
            .cumulative
            .checked_sub(proof.prev_cumulative)
            .ok_or(ErrorCode::InvalidCumulativeRange)?;
        require!(
            expected_balance == proof.balance as u128,
            ErrorCode::BalanceMismatch
        );

        // 4. Verify cumulative doesn't exceed total weight
        require!(
            proof.cumulative <= lottery.total_weight,
            ErrorCode::CumulativeExceedsTotalWeight
        );

        // 5. Compute the leaf hash
        let leaf_hash = compute_leaf_hash(
            &proof.address,
            proof.balance,
            proof.prev_cumulative,
            proof.cumulative,
        );

        // 6. Verify the Merkle proof
        require!(
            verify_proof(leaf_hash, &proof.proof, &lottery.merkle_root),
            ErrorCode::InvalidMerkleProof
        );

        // 7. Compute the selection point from VRF result
        let selection_point = compute_selection_point(&lottery.vrf_result, lottery.total_weight);

        msg!("Selection point: {}", selection_point);
        msg!(
            "Claimant range: [{}, {})",
            proof.prev_cumulative,
            proof.cumulative
        );

        // 8. Verify the selection point falls within the claimant's range
        require!(
            is_winner(selection_point, proof.prev_cumulative, proof.cumulative),
            ErrorCode::NotTheWinner
        );

        // 9. Set the winner and finalize
        lottery.winner = Some(proof.address);
        lottery.winner_weight = proof.balance;
        lottery.finalized = true;

        msg!("Winner claimed successfully!");
        msg!("Winner: {}", proof.address);
        msg!("Winner weight: {}", proof.balance);

        Ok(())
    }
}
