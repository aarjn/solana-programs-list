# Merkle Proof Explained Like You're 5

## Table of Contents
1. [The Problem We're Solving](#the-problem-were-solving)
2. [What is a Hash?](#what-is-a-hash)
3. [What is a Merkle Tree?](#what-is-a-merkle-tree)
4. [Building a Merkle Tree Step by Step](#building-a-merkle-tree-step-by-step)
5. [What is a Merkle Proof?](#what-is-a-merkle-proof)
6. [Generating a Proof](#generating-a-proof)
7. [Verifying a Proof On-Chain](#verifying-a-proof-on-chain)
8. [The Complete Lottery Flow](#the-complete-lottery-flow)
9. [Why This is Secure](#why-this-is-secure)
10. [Code Walkthrough](#code-walkthrough)

---

## The Problem We're Solving

Imagine you have a lottery with 10,000 token holders. You need to:

1. **Prove** someone is in the list
2. **Prove** their exact token balance
3. Do this **on-chain** without storing 10,000 entries (too expensive!)

**Solution**: Store just ONE hash (32 bytes) that represents ALL 10,000 entries.

```
❌ Bad: Store 10,000 addresses on-chain = ~320,000 bytes = $$$

✅ Good: Store 1 merkle root on-chain = 32 bytes = cheap!
```

---

## What is a Hash?

A hash is like a **fingerprint** for data.

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   INPUT (any size)              OUTPUT (fixed size)            │
│   ─────────────────             ─────────────────              │
│                                                                 │
│   "Hello"           ────►       0x2cf24dba...                  │
│   "Hello!"          ────►       0x33b93717...  (totally different!)
│   "War and Peace"   ────►       0x8a9bc127...  (still 32 bytes)│
│   (entire book)                                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Properties:**

| Property | Meaning | Example |
|----------|---------|---------|
| Deterministic | Same input → same output | hash("cat") always = 0xABC |
| One-way | Can't reverse | 0xABC → ??? (impossible) |
| Unique | Different input → different output | hash("cat") ≠ hash("cat!") |
| Fixed size | Output always 32 bytes | Any input → 32 bytes |

**Simple Analogy**: A hash is like baking a cake.
- Recipe (input) → Cake (hash)
- Same recipe always makes same cake
- You can't "unbake" a cake back to ingredients
- Change one ingredient slightly → completely different cake

---

## What is a Merkle Tree?

A Merkle tree is a **tree of hashes** where:
- **Leaves** = hashes of actual data
- **Branches** = hashes of their children combined
- **Root** = single hash representing EVERYTHING

```
                           ┌─────────┐
                           │  ROOT   │ ← One hash for entire tree
                           │ 0xROOT │
                           └────┬────┘
                                │
                 ┌──────────────┴──────────────┐
                 │                             │
            ┌────┴────┐                   ┌────┴────┐
            │  H(0,1) │                   │  H(2,3) │
            │ 0xAB12  │                   │ 0xCD34  │
            └────┬────┘                   └────┬────┘
                 │                             │
         ┌───────┴───────┐             ┌───────┴───────┐
         │               │             │               │
    ┌────┴────┐     ┌────┴────┐   ┌────┴────┐     ┌────┴────┐
    │   H(0)  │     │   H(1)  │   │   H(2)  │     │   H(3)  │
    │ 0x1111  │     │ 0x2222  │   │ 0x3333  │     │ 0x4444  │
    └────┬────┘     └────┬────┘   └────┬────┘     └────┬────┘
         │               │             │               │
    ┌────┴────┐     ┌────┴────┐   ┌────┴────┐     ┌────┴────┐
    │  Alice  │     │   Bob   │   │  Carol  │     │  Dave   │
    │  1000   │     │  3000   │   │  2000   │     │  4000   │
    └─────────┘     └─────────┘   └─────────┘     └─────────┘

         LEAVES (actual data)
```

**Simple Analogy**: Think of a sports tournament bracket, but upside down.

```
Championship Game (ROOT)
        │
   ┌────┴────┐
Semi-Final  Semi-Final
   │            │
┌──┴──┐      ┌──┴──┐
QF    QF     QF    QF
│     │      │     │
Players (LEAVES)
```

---

## Building a Merkle Tree Step by Step

Let's build a tree for our lottery holders:

### Step 1: Start with Raw Data

```
Holder List:
┌─────────┬─────────┬─────────────┬─────────────┐
│ Address │ Balance │ PrevCumul   │ Cumulative  │
├─────────┼─────────┼─────────────┼─────────────┤
│ Alice   │ 1000    │ 0           │ 1000        │
│ Bob     │ 3000    │ 1000        │ 4000        │
│ Carol   │ 2000    │ 4000        │ 6000        │
│ Dave    │ 4000    │ 6000        │ 10000       │
└─────────┴─────────┴─────────────┴─────────────┘
```

### Step 2: Hash Each Entry (Create Leaves)

```javascript
// For each holder, create a leaf hash:
leaf = keccak256(address + balance + prevCumulative + cumulative)
```

```
Alice's leaf:
  Input:  "Alice" + 1000 + 0 + 1000
  Hash:   0x1111111111111111111111111111111111111111111111111111111111111111

Bob's leaf:
  Input:  "Bob" + 3000 + 1000 + 4000
  Hash:   0x2222222222222222222222222222222222222222222222222222222222222222

Carol's leaf:
  Input:  "Carol" + 2000 + 4000 + 6000
  Hash:   0x3333333333333333333333333333333333333333333333333333333333333333

Dave's leaf:
  Input:  "Dave" + 4000 + 6000 + 10000
  Hash:   0x4444444444444444444444444444444444444444444444444444444444444444
```

### Step 3: Combine Pairs (Build Branches)

```javascript
// Combine adjacent leaves
H(Alice, Bob) = keccak256(sort(0x1111, 0x2222))
H(Carol, Dave) = keccak256(sort(0x3333, 0x4444))
```

```
Level 1 (Leaves):
  0x1111    0x2222    0x3333    0x4444
     └────┬────┘         └────┬────┘
          │                   │
Level 2 (Branches):
       0xAB12              0xCD34
          └────────┬────────┘
                   │
Level 3 (Root):
               0xROOT
```

### Step 4: Final Root

```
ROOT = keccak256(sort(0xAB12, 0xCD34))
ROOT = 0xROOT...

This ONE hash represents ALL 4 holders!
```

---

## What is a Merkle Proof?

A Merkle proof is the **minimum set of hashes** needed to prove a leaf belongs to the tree.

**Analogy**: Imagine proving you're in a family photo without showing the whole photo.

```
Full photo (expensive to share):
┌─────────────────────────┐
│ 👨 👩 👦 👧 👴 👵 🐕 🐈 │
└─────────────────────────┘

Proof you're in it (cheap):
"I'm the 👦, and here's proof:
 - 👧 is next to me
 - 👨👩 are our parents
 - That matches the family tree root"
```

For Bob to prove he's in the Merkle tree:

```
                           ┌─────────┐
                           │  ROOT   │ ← Stored on-chain (known)
                           │ 0xROOT │
                           └────┬────┘
                                │
                 ┌──────────────┴──────────────┐
                 │                             │
            ┌────┴────┐                   ┌────┴────┐
            │  H(0,1) │                   │ H(2,3)  │ ← PROOF[1]
            │ (calc)  │                   │ 0xCD34  │
            └────┬────┘                   └─────────┘
                 │
         ┌───────┴───────┐
         │               │
    ┌────┴────┐     ┌────┴────┐
    │  H(0)   │     │  H(1)   │
    │ 0x1111  │     │  (me!)  │
    │ PROOF[0]│     │         │
    └─────────┘     └─────────┘
                         │
                    ┌────┴────┐
                    │   Bob   │ ← Trying to prove this
                    │  3000   │
                    └─────────┘

Bob's proof = [0x1111, 0xCD34]  (just 2 hashes!)
```

---

## Generating a Proof

### The Algorithm (Simple Version)

```
To generate proof for Bob (index 1):

1. Start at Bob's leaf
2. Walk up to root, collecting SIBLING hashes

         ROOT
        /    \
     H(0,1)  [H(2,3)] ← Collect this (sibling of my path)
     /    \
  [H(0)]  H(1)=me ← Collect this (sibling of me)
           |
          Bob

Proof = [H(0), H(2,3)] = [0x1111, 0xCD34]
```

### In Code (TypeScript)

```typescript
// From merkle.ts

getProof(index: number): Buffer[] {
  const proof: Buffer[] = [];
  let currentIndex = index;
  let nodes = this.leaves;  // Start with all leaves

  while (nodes.length > 1) {
    const nextLevel = [];

    for (let i = 0; i < nodes.length; i += 2) {
      if (i + 1 < nodes.length) {
        // Has a sibling
        if (i === currentIndex || i + 1 === currentIndex) {
          // This is my level - collect sibling!
          const siblingIndex = currentIndex === i ? i + 1 : i;
          proof.push(nodes[siblingIndex]);
          currentIndex = Math.floor(currentIndex / 2);
        }
        // Combine for next level
        nextLevel.push(hashPair(nodes[i], nodes[i + 1]));
      } else {
        // Odd node, promote
        nextLevel.push(nodes[i]);
      }
    }
    nodes = nextLevel;
  }

  return proof;
}
```

### Visual Example: Generating Bob's Proof

```
LEVEL 0 (Leaves):
Index:    0         1         2         3
       [Alice]   [Bob]    [Carol]   [Dave]
        0x1111   0x2222    0x3333    0x4444
                   ↑
              currentIndex = 1

My sibling is index 0 (Alice)
proof.push(0x1111) ✓
currentIndex = floor(1/2) = 0

───────────────────────────────────────────────

LEVEL 1 (Branches):
Index:      0              1
        [H(0,1)]       [H(2,3)]
         0xAB12         0xCD34
            ↑
       currentIndex = 0

My sibling is index 1
proof.push(0xCD34) ✓
currentIndex = floor(0/2) = 0

───────────────────────────────────────────────

LEVEL 2 (Root):
Only 1 node, we're done!

FINAL PROOF = [0x1111, 0xCD34]
```

---

## Verifying a Proof On-Chain

This is where the magic happens. The on-chain program:
1. Takes the proof
2. Recomputes the root
3. Compares with stored root

### The Algorithm

```
To verify Bob's proof [0x1111, 0xCD34]:

1. Compute Bob's leaf hash
   leaf = hash(Bob + 3000 + 1000 + 4000) = 0x2222

2. Combine with proof[0]
   step1 = hash(sort(0x2222, 0x1111)) = 0xAB12

3. Combine with proof[1]
   step2 = hash(sort(0xAB12, 0xCD34)) = 0xROOT

4. Compare
   step2 == stored_root?
   0xROOT == 0xROOT? ✓ YES!

Bob is verified!
```

### Visual Verification

```
                    VERIFICATION PROCESS

Bob claims: "I'm in the tree with balance 3000, range [1000, 4000)"
Bob provides: proof = [0x1111, 0xCD34]

Step 1: Compute leaf
┌─────────────────────────────────────────────────────────┐
│ leaf = keccak256("Bob" + 3000 + 1000 + 4000)           │
│ leaf = 0x2222                                           │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
Step 2: Combine with proof[0]
┌─────────────────────────────────────────────────────────┐
│ Input: leaf=0x2222, sibling=0x1111                     │
│ Sort:  [0x1111, 0x2222]  (smaller first)               │
│ Hash:  keccak256(0x1111 + 0x2222)                      │
│ Result: 0xAB12                                          │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
Step 3: Combine with proof[1]
┌─────────────────────────────────────────────────────────┐
│ Input: current=0xAB12, sibling=0xCD34                  │
│ Sort:  [0xAB12, 0xCD34]                                │
│ Hash:  keccak256(0xAB12 + 0xCD34)                      │
│ Result: 0xROOT                                          │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
Step 4: Compare with stored root
┌─────────────────────────────────────────────────────────┐
│ Computed: 0xROOT                                        │
│ Stored:   0xROOT (from lottery account)                │
│                                                         │
│ MATCH? ✓ YES!                                          │
│                                                         │
│ Bob is VERIFIED to be in the original list!            │
└─────────────────────────────────────────────────────────┘
```

### In Code (Rust - On-Chain)

```rust
// From merkle.rs

pub fn verify_proof(leaf: [u8; 32], proof: &[[u8; 32]], root: &[u8; 32]) -> bool {
    let mut current = leaf;

    // Walk up the tree
    for sibling in proof.iter() {
        current = hash_pair(&current, sibling);
    }

    // Compare with stored root
    current == *root
}

// Helper: hash two nodes together (sorted for consistency)
pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut combined = Vec::with_capacity(64);

    // Always sort to ensure same result regardless of order
    if left <= right {
        combined.extend_from_slice(left);
        combined.extend_from_slice(right);
    } else {
        combined.extend_from_slice(right);
        combined.extend_from_slice(left);
    }

    keccak256(&combined)
}
```

---

## The Complete Lottery Flow

Now let's see how Merkle proofs fit into the lottery:

```
┌─────────────────────────────────────────────────────────────────┐
│                        SETUP PHASE                              │
└─────────────────────────────────────────────────────────────────┘

1. SNAPSHOT: Query all token holders

   holders.json:
   {
     "Alice": 1000,
     "Bob": 3000,
     "Carol": 2000,
     "Dave": 4000
   }

2. BUILD TREE: Create merkle tree with cumulative weights

   ┌─────────┬─────────┬──────────┬────────────┐
   │ Address │ Balance │ PrevCum  │ Cumulative │
   ├─────────┼─────────┼──────────┼────────────┤
   │ Alice   │ 1000    │ 0        │ 1000       │
   │ Bob     │ 3000    │ 1000     │ 4000       │ ← range [1000, 4000)
   │ Carol   │ 2000    │ 4000     │ 6000       │
   │ Dave    │ 4000    │ 6000     │ 10000      │
   └─────────┴─────────┴──────────┴────────────┘

   Merkle Root = 0xROOT

3. INITIALIZE: Store root on-chain

   Lottery PDA:
   ┌─────────────────────────┐
   │ merkle_root: 0xROOT    │ ← Locked!
   │ total_weight: 10000     │
   │ vrf_result: (empty)     │
   │ winner: None            │
   └─────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                       RANDOMNESS PHASE                          │
└─────────────────────────────────────────────────────────────────┘

4. VRF: Get random number from Switchboard

   vrf_result = 0x30d56284e4665c19...

   Lottery PDA:
   ┌─────────────────────────┐
   │ merkle_root: 0xROOT    │
   │ total_weight: 10000     │
   │ vrf_result: 0x30d5...  │ ← Set!
   │ winner: None            │
   └─────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                        CLAIM PHASE                              │
└─────────────────────────────────────────────────────────────────┘

5. COMPUTE WINNER (anyone can do this):

   selection_point = vrf_result % total_weight
   selection_point = 0x30d5... % 10000
   selection_point = 2500

   Who has 2500 in their range?
   - Alice: [0, 1000)      → 2500 not here
   - Bob:   [1000, 4000)   → 2500 IS HERE! ✓
   - Carol: [4000, 6000)   → 2500 not here
   - Dave:  [6000, 10000)  → 2500 not here

   WINNER = Bob

6. GENERATE PROOF (Bob or anyone):

   proof = tree.getProof(bob_index)
   proof = [0x1111, 0xCD34]  // Alice's hash, Carol+Dave hash

7. CLAIM (Bob signs transaction):

   claimWinner({
     address: Bob,
     balance: 3000,
     prevCumulative: 1000,
     cumulative: 4000,
     proof: [0x1111, 0xCD34]
   })

8. ON-CHAIN VERIFICATION:

   ┌─────────────────────────────────────────────────────────────┐
   │ CHECK 1: Is signer Bob?                                     │
   │          transaction.signer == "Bob"? ✓                     │
   │                                                             │
   │ CHECK 2: Is balance consistent?                             │
   │          4000 - 1000 == 3000? ✓                             │
   │                                                             │
   │ CHECK 3: Is merkle proof valid?                             │
   │          leaf = hash(Bob + 3000 + 1000 + 4000)             │
   │          computed_root = verify(leaf, proof)               │
   │          computed_root == stored_root? ✓                    │
   │                                                             │
   │ CHECK 4: Did VRF select Bob's range?                        │
   │          selection = vrf % total_weight = 2500             │
   │          2500 >= 1000? ✓                                    │
   │          2500 < 4000? ✓                                     │
   │                                                             │
   │ ALL CHECKS PASSED!                                          │
   └─────────────────────────────────────────────────────────────┘

9. WRITE WINNER:

   Lottery PDA:
   ┌─────────────────────────┐
   │ merkle_root: 0xROOT    │
   │ total_weight: 10000     │
   │ vrf_result: 0x30d5...  │
   │ winner: Bob            │ ← Written!
   │ finalized: true         │
   └─────────────────────────┘
```

---

## Why This is Secure

### Attack 1: Fake Being in the List

```
Eve (not in list) tries to claim:

Eve submits:
{
  address: Eve,
  balance: 5000,
  range: [2000, 7000),
  proof: [some_fake_hashes]
}

Verification:
  leaf = hash(Eve + 5000 + 2000 + 7000)
  computed_root = verify(leaf, fake_proof)
  computed_root = 0xGARBAGE

  0xGARBAGE == 0xROOT? ❌ NO!

REJECTED - Eve is not in the tree
```

### Attack 2: Lie About Balance

```
Alice (balance=1000) claims she has 5000:

Alice submits:
{
  address: Alice,
  balance: 5000,      ← Lie!
  range: [0, 5000),   ← Lie!
  proof: [real_proof]
}

Verification:
  leaf = hash(Alice + 5000 + 0 + 5000)  ← Wrong data!
  computed_root = 0xWRONG

  0xWRONG == 0xROOT? ❌ NO!

REJECTED - Hash doesn't match because data was changed
```

### Attack 3: Claim When Not Winner

```
Carol (range [4000, 6000)) claims when selection=2500:

Carol submits:
{
  address: Carol,
  balance: 2000,
  range: [4000, 6000),
  proof: [valid_proof]
}

Verification:
  Merkle proof: ✓ Valid (Carol IS in tree)

  But...
  selection_point = 2500
  2500 >= 4000? ❌ NO!

REJECTED - VRF didn't pick Carol's range
```

### Attack 4: Admin Changes List After VRF

```
Admin sees VRF picked range [1000, 4000) (Bob wins)
Admin wants Alice to win instead

Admin publishes new list:
{
  Alice: 5000,  ← Changed to cover [1000, 6000)
  Carol: 2000,
  Dave: 2000
}

New merkle root = 0xNEW

But on-chain root is still 0xROOT (set before VRF)

Alice claims with proof from new list:
  computed_root = 0xNEW
  0xNEW == 0xROOT? ❌ NO!

REJECTED - Can't change the list after it's committed
```

---

## Code Walkthrough

### 1. Leaf Hash Computation

**TypeScript (off-chain):**
```typescript
// merkle.ts
export function computeLeafHash(entry: HolderEntry): Buffer {
  const data = Buffer.concat([
    entry.address.toBuffer(),              // 32 bytes
    u64ToLeBytes(entry.balance),           // 8 bytes
    u128ToLeBytes(entry.prevCumulative),   // 16 bytes
    u128ToLeBytes(entry.cumulative),       // 16 bytes
  ]);
  return keccak256(data);  // Returns 32 bytes
}
```

**Rust (on-chain):**
```rust
// merkle.rs
pub fn compute_leaf_hash(
    address: &Pubkey,
    balance: u64,
    prev_cumulative: u128,
    cumulative: u128,
) -> [u8; 32] {
    let mut data = Vec::with_capacity(72);
    data.extend_from_slice(address.as_ref());           // 32 bytes
    data.extend_from_slice(&balance.to_le_bytes());     // 8 bytes
    data.extend_from_slice(&prev_cumulative.to_le_bytes()); // 16 bytes
    data.extend_from_slice(&cumulative.to_le_bytes());  // 16 bytes
    keccak256(&data)
}
```

**Both MUST produce identical output for same input!**

### 2. Hash Pair (For Building/Verifying Tree)

```rust
// merkle.rs
pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut combined = Vec::with_capacity(64);

    // IMPORTANT: Sort for deterministic ordering
    // This ensures hash(A,B) == hash(B,A)
    if left <= right {
        combined.extend_from_slice(left);
        combined.extend_from_slice(right);
    } else {
        combined.extend_from_slice(right);
        combined.extend_from_slice(left);
    }

    keccak256(&combined)
}
```

### 3. Proof Verification

```rust
// merkle.rs
pub fn verify_proof(
    leaf: [u8; 32],
    proof: &[[u8; 32]],
    root: &[u8; 32]
) -> bool {
    let mut current = leaf;

    for sibling in proof.iter() {
        current = hash_pair(&current, sibling);
    }

    current == *root
}
```

### 4. Complete Claim Verification

```rust
// claim_winner.rs
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
    let expected_balance = proof.cumulative - proof.prev_cumulative;
    require!(
        expected_balance == proof.balance as u128,
        ErrorCode::BalanceMismatch
    );

    // 4. Compute the leaf hash
    let leaf_hash = compute_leaf_hash(
        &proof.address,
        proof.balance,
        proof.prev_cumulative,
        proof.cumulative,
    );

    // 5. Verify the Merkle proof
    require!(
        verify_proof(leaf_hash, &proof.proof, &lottery.merkle_root),
        ErrorCode::InvalidMerkleProof
    );

    // 6. Compute the selection point from VRF result
    let selection_point = compute_selection_point(
        &lottery.vrf_result,
        lottery.total_weight
    );

    // 7. Verify the selection point falls within the claimant's range
    require!(
        is_winner(selection_point, proof.prev_cumulative, proof.cumulative),
        ErrorCode::NotTheWinner
    );

    // 8. All checks passed - record winner
    lottery.winner = Some(proof.address);
    lottery.winner_weight = proof.balance;
    lottery.finalized = true;

    Ok(())
}
```

---

## Summary

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   MERKLE TREE = Compress 10,000 entries into 1 hash            │
│                                                                 │
│   MERKLE PROOF = Prove you're in the tree with ~14 hashes      │
│                  (log2 of 10,000 ≈ 14)                          │
│                                                                 │
│   ON-CHAIN VERIFICATION = Recompute root from proof            │
│                           Compare with stored root              │
│                           If match → proof is valid             │
│                                                                 │
│   SECURITY = Can't fake proof (wrong hash)                     │
│              Can't change list (root is locked)                │
│              Can't claim wrong range (VRF check fails)         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**The beauty**: Store 32 bytes on-chain, prove membership of any entry with ~14 hashes, verification is pure math that anyone can check.
