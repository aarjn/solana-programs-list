use anchor_lang::prelude::*;
use solana_keccak_hasher::hashv;

/// Compute keccak256 hash of the input
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    hashv(&[data]).0
}

/// Compute the leaf hash for a holder entry
/// leaf = keccak256(address || balance || prev_cumulative || cumulative)
/// All numeric values are little-endian encoded
pub fn compute_leaf_hash(
    address: &Pubkey,
    balance: u64,
    prev_cumulative: u128,
    cumulative: u128,
) -> [u8; 32] {
    let mut data = Vec::with_capacity(32 + 8 + 16 + 16); // 72 bytes
    data.extend_from_slice(address.as_ref());
    data.extend_from_slice(&balance.to_le_bytes());
    data.extend_from_slice(&prev_cumulative.to_le_bytes());
    data.extend_from_slice(&cumulative.to_le_bytes());
    keccak256(&data)
}

/// Compute hash of two sibling nodes
/// For deterministic ordering, we sort the two hashes before hashing
/// This ensures the same root regardless of proof direction
pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut combined = Vec::with_capacity(64);
    // Sort to ensure deterministic ordering
    if left <= right {
        combined.extend_from_slice(left);
        combined.extend_from_slice(right);
    } else {
        combined.extend_from_slice(right);
        combined.extend_from_slice(left);
    }
    keccak256(&combined)
}

/// Verify a Merkle proof
/// Returns true if the proof is valid and the leaf belongs to the tree with given root
pub fn verify_proof(leaf: [u8; 32], proof: &[[u8; 32]], root: &[u8; 32]) -> bool {
    let mut current = leaf;

    for sibling in proof.iter() {
        current = hash_pair(&current, sibling);
    }

    current == *root
}

/// Convert VRF result (32 bytes) to selection point using modulo total_weight
/// Uses first 16 bytes as u128 (little-endian) for the random value
pub fn compute_selection_point(vrf_result: &[u8; 32], total_weight: u128) -> u128 {
    if total_weight == 0 {
        return 0;
    }

    // Use first 16 bytes as u128 (little-endian)
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&vrf_result[..16]);
    let random_value = u128::from_le_bytes(bytes);

    random_value % total_weight
}

/// Check if a selection point falls within the given range [prev_cumulative, cumulative)
pub fn is_winner(selection_point: u128, prev_cumulative: u128, cumulative: u128) -> bool {
    selection_point >= prev_cumulative && selection_point < cumulative
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_pair_ordering() {
        let a = [1u8; 32];
        let b = [2u8; 32];

        // hash_pair should be commutative due to sorting
        assert_eq!(hash_pair(&a, &b), hash_pair(&b, &a));
    }

    #[test]
    fn test_selection_point() {
        let vrf = [0xFF; 32]; // Max value
        let total = 1000u128;
        let point = compute_selection_point(&vrf, total);
        assert!(point < total);
    }

    #[test]
    fn test_is_winner() {
        assert!(is_winner(50, 0, 100));
        assert!(is_winner(0, 0, 100));
        assert!(!is_winner(100, 0, 100)); // Upper bound is exclusive
        assert!(!is_winner(50, 100, 200));
    }
}
