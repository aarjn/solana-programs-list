use anchor_lang::prelude::*;

use crate::{
    hash::ZERO_HASHES,
    state::{MerkleTree, DEPTH},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = MerkleTree::SPACE,
        seeds = [b"merkle", authority.key().as_ref()],
        bump,
    )]
    pub merkle_tree: Account<'info, MerkleTree>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let tree = &mut ctx.accounts.merkle_tree;
    tree.authority = ctx.accounts.authority.key();
    tree.depth = DEPTH as u8;
    tree.next_index = 0;
    tree.current_root = ZERO_HASHES[DEPTH];
    tree.filled_subtrees = core::array::from_fn(|i| ZERO_HASHES[i]);
    Ok(())
}
