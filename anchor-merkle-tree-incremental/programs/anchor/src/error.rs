use anchor_lang::prelude::*;

#[error_code]
pub enum MerkleError {
    #[msg("Leaf cannot be zero")]
    ZeroLeaf,
    #[msg("Merkle tree is full")]
    TreeFull,
    #[msg("Invalid merkle proof")]
    InvalidProof,
}
