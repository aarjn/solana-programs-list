use sha2_const_stable::Sha256 as ConstSha256;

const fn make_zero_hashes() -> [[u8; 32]; 21] {
    let mut h = [[0u8; 32]; 21];
    h[0] = ConstSha256::new().update(&[0u8; 32]).finalize();
    let mut i = 1usize;
    while i <= 20 {
        let mut buf = [0u8; 64];
        let mut j = 0usize;
        while j < 32 {
            buf[j] = h[i - 1][j];
            buf[32 + j] = h[i - 1][j];
            j += 1;
        }
        h[i] = ConstSha256::new().update(&buf).finalize();
        i += 1;
    }
    h
}

pub const ZERO_HASHES: [[u8; 32]; 21] = make_zero_hashes();

#[inline(always)]
pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    solana_sha256_hasher::hashv(&[left.as_ref(), right.as_ref()]).to_bytes()
}
