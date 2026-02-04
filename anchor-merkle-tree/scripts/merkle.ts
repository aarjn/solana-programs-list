import { PublicKey } from "@solana/web3.js";
import { keccak_256 } from "@noble/hashes/sha3";
import BN from "bn.js";

/**
 * Holder entry with cumulative weight information
 */
export interface HolderEntry {
  address: PublicKey;
  balance: bigint;
  prevCumulative: bigint;
  cumulative: bigint;
}

/**
 * Winner proof data for on-chain verification
 */
export interface WinnerProof {
  address: PublicKey;
  balance: bigint;
  prevCumulative: bigint;
  cumulative: bigint;
  proof: Buffer[];
}

/**
 * Merkle tree node
 */
interface MerkleNode {
  hash: Buffer;
  left?: MerkleNode;
  right?: MerkleNode;
  leafIndex?: number;
}

/**
 * Convert a bigint to a 16-byte little-endian Buffer (u128)
 */
function u128ToLeBytes(value: bigint): Buffer {
  const buf = Buffer.alloc(16);
  let v = value;
  for (let i = 0; i < 16; i++) {
    buf[i] = Number(v & 0xffn);
    v >>= 8n;
  }
  return buf;
}

/**
 * Convert a bigint to an 8-byte little-endian Buffer (u64)
 */
function u64ToLeBytes(value: bigint): Buffer {
  const buf = Buffer.alloc(8);
  let v = value;
  for (let i = 0; i < 8; i++) {
    buf[i] = Number(v & 0xffn);
    v >>= 8n;
  }
  return buf;
}

/**
 * Convert first 16 bytes of a buffer to bigint (little-endian u128)
 */
function leBytesToU128(bytes: Buffer | Uint8Array): bigint {
  let result = 0n;
  for (let i = 15; i >= 0; i--) {
    result = (result << 8n) | BigInt(bytes[i]);
  }
  return result;
}

/**
 * Compute keccak256 hash
 */
function keccak256(data: Buffer): Buffer {
  return Buffer.from(keccak_256(data));
}

/**
 * Compute the leaf hash for a holder entry
 * leaf = keccak256(address || balance || prev_cumulative || cumulative)
 */
export function computeLeafHash(entry: HolderEntry): Buffer {
  const data = Buffer.concat([
    entry.address.toBuffer(),
    u64ToLeBytes(entry.balance),
    u128ToLeBytes(entry.prevCumulative),
    u128ToLeBytes(entry.cumulative),
  ]);
  return keccak256(data);
}

/**
 * Hash two sibling nodes together (sorted for deterministic ordering)
 */
function hashPair(left: Buffer, right: Buffer): Buffer {
  // Sort to ensure deterministic ordering (matches on-chain logic)
  const sorted =
    Buffer.compare(left, right) <= 0 ? [left, right] : [right, left];
  return keccak256(Buffer.concat(sorted));
}

/**
 * Build holder entries from raw holder data with cumulative weights
 * Holders are sorted by address for deterministic ordering
 */
export function buildHolderEntries(
  holders: Record<string, string | number>
): HolderEntry[] {
  // Convert to array and sort by address
  const sortedHolders = Object.entries(holders)
    .map(([address, balance]) => ({
      address: new PublicKey(address),
      balance: BigInt(balance),
    }))
    .sort((a, b) => a.address.toBuffer().compare(b.address.toBuffer()));

  // Build entries with cumulative weights
  let cumulative = 0n;
  const entries: HolderEntry[] = [];

  for (const holder of sortedHolders) {
    const prevCumulative = cumulative;
    cumulative += holder.balance;

    entries.push({
      address: holder.address,
      balance: holder.balance,
      prevCumulative,
      cumulative,
    });
  }

  return entries;
}

/**
 * Merkle Tree class for weighted lottery
 */
export class MerkleTree {
  private root: MerkleNode;
  private leaves: Buffer[];
  private entries: HolderEntry[];
  private totalWeight: bigint;

  constructor(entries: HolderEntry[]) {
    if (entries.length === 0) {
      throw new Error("Cannot create Merkle tree with no entries");
    }

    this.entries = entries;
    this.totalWeight = entries[entries.length - 1].cumulative;
    this.leaves = entries.map(computeLeafHash);
    this.root = this.buildTree(this.leaves);
  }

  /**
   * Build the Merkle tree recursively
   */
  private buildTree(leaves: Buffer[]): MerkleNode {
    if (leaves.length === 1) {
      return { hash: leaves[0], leafIndex: 0 };
    }

    // Build nodes array with leaf indices
    let nodes: MerkleNode[] = leaves.map((hash, i) => ({
      hash,
      leafIndex: i,
    }));

    // Build tree level by level
    while (nodes.length > 1) {
      const nextLevel: MerkleNode[] = [];

      for (let i = 0; i < nodes.length; i += 2) {
        if (i + 1 < nodes.length) {
          // Pair exists
          const combined = hashPair(nodes[i].hash, nodes[i + 1].hash);
          nextLevel.push({
            hash: combined,
            left: nodes[i],
            right: nodes[i + 1],
          });
        } else {
          // Odd node, promote to next level
          nextLevel.push(nodes[i]);
        }
      }

      nodes = nextLevel;
    }

    return nodes[0];
  }

  /**
   * Get the Merkle root as a 32-byte array
   */
  getRoot(): Buffer {
    return this.root.hash;
  }

  /**
   * Get the Merkle root as a number array for Anchor
   */
  getRootArray(): number[] {
    return Array.from(this.root.hash);
  }

  /**
   * Get total weight
   */
  getTotalWeight(): bigint {
    return this.totalWeight;
  }

  /**
   * Get all entries
   */
  getEntries(): HolderEntry[] {
    return this.entries;
  }

  /**
   * Generate Merkle proof for a leaf at given index
   */
  getProof(index: number): Buffer[] {
    if (index < 0 || index >= this.leaves.length) {
      throw new Error(`Invalid leaf index: ${index}`);
    }

    const proof: Buffer[] = [];
    let currentIndex = index;
    let nodes = this.leaves.map((hash) => ({ hash }));

    while (nodes.length > 1) {
      const nextLevel: { hash: Buffer }[] = [];

      for (let i = 0; i < nodes.length; i += 2) {
        if (i + 1 < nodes.length) {
          // Has sibling
          if (i === currentIndex || i + 1 === currentIndex) {
            // This is our node's level, add sibling to proof
            const siblingIndex = currentIndex === i ? i + 1 : i;
            proof.push(nodes[siblingIndex].hash);
            currentIndex = Math.floor(currentIndex / 2);
          }

          const combined = hashPair(nodes[i].hash, nodes[i + 1].hash);
          nextLevel.push({ hash: combined });
        } else {
          // Odd node
          if (i === currentIndex) {
            // We're the odd one, no sibling needed for this level
            currentIndex = Math.floor(currentIndex / 2);
          }
          nextLevel.push(nodes[i]);
        }
      }

      nodes = nextLevel;
    }

    return proof;
  }

  /**
   * Find the winner entry given a VRF result
   * Uses binary search for efficiency
   */
  findWinner(vrfResult: Buffer | Uint8Array): { entry: HolderEntry; index: number } {
    // Compute selection point: (vrf as u128) % total_weight
    const vrfValue = leBytesToU128(vrfResult);
    const selectionPoint = vrfValue % this.totalWeight;

    // Binary search for the entry containing the selection point
    let left = 0;
    let right = this.entries.length - 1;

    while (left < right) {
      const mid = Math.floor((left + right) / 2);
      const entry = this.entries[mid];

      if (entry.cumulative <= selectionPoint) {
        left = mid + 1;
      } else {
        right = mid;
      }
    }

    const winner = this.entries[left];

    // Verify the selection point falls within the winner's range
    if (selectionPoint < winner.prevCumulative || selectionPoint >= winner.cumulative) {
      throw new Error(
        `Binary search error: selection point ${selectionPoint} not in range [${winner.prevCumulative}, ${winner.cumulative})`
      );
    }

    return { entry: winner, index: left };
  }

  /**
   * Generate winner proof for on-chain submission
   */
  generateWinnerProof(vrfResult: Buffer | Uint8Array): WinnerProof {
    const { entry, index } = this.findWinner(vrfResult);
    const proof = this.getProof(index);

    return {
      address: entry.address,
      balance: entry.balance,
      prevCumulative: entry.prevCumulative,
      cumulative: entry.cumulative,
      proof,
    };
  }

  /**
   * Verify a proof against the root
   */
  verifyProof(entry: HolderEntry, proof: Buffer[]): boolean {
    let current = computeLeafHash(entry);

    for (const sibling of proof) {
      current = hashPair(current, sibling);
    }

    return current.equals(this.root.hash);
  }
}

/**
 * Convert WinnerProof to format suitable for Anchor instruction
 */
export function proofToAnchorFormat(proof: WinnerProof): {
  address: PublicKey;
  balance: BN;
  prevCumulative: BN;
  cumulative: BN;
  proof: number[][];
} {
  return {
    address: proof.address,
    balance: new BN(proof.balance.toString()),
    prevCumulative: new BN(proof.prevCumulative.toString()),
    cumulative: new BN(proof.cumulative.toString()),
    proof: proof.proof.map((p) => Array.from(p)),
  };
}

/**
 * Compute selection point from VRF result (for debugging/verification)
 */
export function computeSelectionPoint(
  vrfResult: Buffer | Uint8Array,
  totalWeight: bigint
): bigint {
  const vrfValue = leBytesToU128(vrfResult);
  return vrfValue % totalWeight;
}
