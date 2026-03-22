use anchor_lang::prelude::*;

use crate::{
    error::MerkleError,
    hash::{hash_pair, ZERO_HASHES},
    state::MerkleTree,
};

#[derive(Accounts)]
pub struct Insert<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"merkle", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub merkle_tree: Account<'info, MerkleTree>,
}

pub fn handler(ctx: Context<Insert>, leaf: [u8; 32]) -> Result<()> {
    require!(leaf != [0u8; 32], MerkleError::ZeroLeaf);

    let tree = &mut ctx.accounts.merkle_tree;
    let max_leaves = 1u32 << tree.depth;
    require!(tree.next_index < max_leaves, MerkleError::TreeFull);

    let mut current_index = tree.next_index;
    let mut current_hash = leaf;
    let depth = tree.depth as usize;

    for i in 0..depth {
        if current_index % 2 == 0 {
            tree.filled_subtrees[i] = current_hash;
            current_hash = hash_pair(&current_hash, &ZERO_HASHES[i]);
        } else {
            current_hash = hash_pair(&tree.filled_subtrees[i], &current_hash);
        }
        current_index >>= 1;
    }

    tree.current_root = current_hash;
    tree.next_index += 1;
    Ok(())
}
