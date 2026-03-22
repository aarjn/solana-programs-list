use anchor_lang::prelude::*;

use crate::{error::MerkleError, hash::hash_pair, state::MerkleTree};

#[derive(Accounts)]
pub struct Verify<'info> {
    pub merkle_tree: Account<'info, MerkleTree>,
}

pub fn handler(
    ctx: Context<Verify>,
    leaf: [u8; 32],
    index: u32,
    proof: [[u8; 32]; 20],
) -> Result<()> {
    let tree = &ctx.accounts.merkle_tree;
    require!(index < tree.next_index, MerkleError::InvalidProof);
    let mut current_hash = leaf;
    let mut current_index = index;
    let depth = tree.depth as usize;

    for i in 0..depth {
        current_hash = if current_index % 2 == 0 {
            hash_pair(&current_hash, &proof[i])
        } else {
            hash_pair(&proof[i], &current_hash)
        };
        current_index >>= 1;
    }

    require!(current_hash == tree.current_root, MerkleError::InvalidProof);
    Ok(())
}
