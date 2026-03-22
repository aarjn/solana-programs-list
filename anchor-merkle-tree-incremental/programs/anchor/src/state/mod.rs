use anchor_lang::prelude::*;

pub const DEPTH: usize = 20;

#[account]
pub struct MerkleTree {
    pub authority: Pubkey,
    pub depth: u8,
    pub next_index: u32,
    pub current_root: [u8; 32],
    pub filled_subtrees: [[u8; 32]; DEPTH],
}

impl MerkleTree {
    // 8 (discriminator) + 32 + 1 + 4 + 32 + (32 * 20)
    pub const SPACE: usize = 8 + 32 + 1 + 4 + 32 + (32 * DEPTH);
}
