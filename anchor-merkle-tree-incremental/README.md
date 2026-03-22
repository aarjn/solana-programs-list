# Anchor Merkle Tree (Incremental)

An on-chain incremental Merkle tree program built with [Anchor](https://www.anchor-lang.com/) on Solana.

### Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize` | Creates the Merkle tree PDA and sets up initial zero-hash state |
| `insert` | Inserts a 32-byte leaf into the tree and recomputes the root |
| `verify` | Verifies a Merkle proof (`[[u8; 32]; 20]`) for a given leaf at a given index |
