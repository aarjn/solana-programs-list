/**
 * Merkle Tree Tests
 * Run with: npx ts-node scripts/merkle.test.ts
 */

import { PublicKey, Keypair } from "@solana/web3.js";
import {
  MerkleTree,
  buildHolderEntries,
  computeLeafHash,
  computeSelectionPoint,
  proofToAnchorFormat,
} from "./merkle";

function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(`Assertion failed: ${message}`);
  }
}

// Generate deterministic addresses for testing
function generateTestAddress(seed: number): string {
  const buf = Buffer.alloc(32, 0);
  buf.writeUInt32BE(seed, 28);
  return new PublicKey(buf).toBase58();
}

function testBuildHolderEntries() {
  console.log("Testing buildHolderEntries...");

  // Use valid Solana public keys (generated from deterministic buffers)
  const holders = {
    [generateTestAddress(1)]: "1000",
    [generateTestAddress(2)]: "500",
    [generateTestAddress(3)]: "2500",
  };

  const entries = buildHolderEntries(holders);

  // Should be sorted by address
  assert(entries.length === 3, "Should have 3 entries");

  // Verify cumulative weights
  let expectedCumulative = BigInt(0);
  for (const entry of entries) {
    assert(
      entry.prevCumulative === expectedCumulative,
      `prevCumulative should be ${expectedCumulative}, got ${entry.prevCumulative}`
    );
    expectedCumulative += entry.balance;
    assert(
      entry.cumulative === expectedCumulative,
      `cumulative should be ${expectedCumulative}, got ${entry.cumulative}`
    );
  }

  console.log("  ✓ Entries built correctly with cumulative weights");

  // Verify sorting (addresses should be in ascending order)
  for (let i = 1; i < entries.length; i++) {
    const cmp = entries[i - 1].address
      .toBuffer()
      .compare(entries[i].address.toBuffer());
    assert(cmp < 0, "Entries should be sorted by address");
  }

  console.log("  ✓ Entries sorted by address");
}

function testMerkleTree() {
  console.log("\nTesting MerkleTree...");

  const holders = {
    [generateTestAddress(1)]: "1000",
    [generateTestAddress(2)]: "500",
    [generateTestAddress(3)]: "2500",
    [generateTestAddress(4)]: "750",
    [generateTestAddress(5)]: "1250",
  };

  const entries = buildHolderEntries(holders);
  const tree = new MerkleTree(entries);

  // Test root
  const root = tree.getRoot();
  assert(root.length === 32, "Root should be 32 bytes");
  console.log("  ✓ Root computed:", root.toString("hex").slice(0, 16) + "...");

  // Test total weight
  const totalWeight = tree.getTotalWeight();
  assert(totalWeight === BigInt(6000), `Total weight should be 6000, got ${totalWeight}`);
  console.log("  ✓ Total weight:", totalWeight.toString());

  // Test proof generation and verification for each entry
  for (let i = 0; i < entries.length; i++) {
    const proof = tree.getProof(i);
    const verified = tree.verifyProof(entries[i], proof);
    assert(verified, `Proof for entry ${i} should be valid`);
  }
  console.log("  ✓ All proofs verified");
}

function testFindWinner() {
  console.log("\nTesting findWinner...");

  const holders = {
    [generateTestAddress(1)]: "1000", // [0, 1000)
    [generateTestAddress(2)]: "500", // [1000, 1500)
    [generateTestAddress(3)]: "2500", // [1500, 4000)
    [generateTestAddress(4)]: "750", // [4000, 4750)
    [generateTestAddress(5)]: "1250", // [4750, 6000)
  };

  const entries = buildHolderEntries(holders);
  const tree = new MerkleTree(entries);

  // Test with known VRF values
  // VRF result where first 16 bytes = 0 should select first entry
  const vrf0 = Buffer.alloc(32, 0);
  const result0 = tree.findWinner(vrf0);
  const selectionPoint0 = computeSelectionPoint(vrf0, tree.getTotalWeight());
  assert(selectionPoint0 === BigInt(0), `Selection point should be 0, got ${selectionPoint0}`);
  console.log(
    `  ✓ VRF=0 -> Selection point ${selectionPoint0} -> Winner at index ${result0.index}`
  );

  // Test that winner is found correctly based on cumulative ranges
  const vrfResult = result0.entry;
  assert(
    selectionPoint0 >= vrfResult.prevCumulative &&
      selectionPoint0 < vrfResult.cumulative,
    "Selection point should be in winner's range"
  );
  console.log("  ✓ Winner range verified");

  // Test with VRF that gives selection point in middle
  const vrf2000 = Buffer.alloc(32, 0);
  vrf2000.writeUInt32LE(2000, 0);
  const selectionPoint2000 = computeSelectionPoint(vrf2000, tree.getTotalWeight());
  const result2000 = tree.findWinner(vrf2000);
  assert(
    selectionPoint2000 === BigInt(2000),
    `Selection point should be 2000, got ${selectionPoint2000}`
  );
  console.log(
    `  ✓ VRF=2000 -> Selection point ${selectionPoint2000} -> Winner balance ${result2000.entry.balance}`
  );
}

function testProofFormat() {
  console.log("\nTesting proofToAnchorFormat...");

  const holders = {
    [generateTestAddress(1)]: "1000",
    [generateTestAddress(2)]: "500",
  };

  const entries = buildHolderEntries(holders);
  const tree = new MerkleTree(entries);

  const vrf = Buffer.alloc(32, 0);
  const winnerProof = tree.generateWinnerProof(vrf);
  const anchorFormat = proofToAnchorFormat(winnerProof);

  assert(anchorFormat.address instanceof PublicKey, "address should be PublicKey");
  assert(typeof anchorFormat.balance.toString === "function", "balance should be BN");
  assert(
    typeof anchorFormat.prevCumulative.toString === "function",
    "prevCumulative should be BN"
  );
  assert(
    typeof anchorFormat.cumulative.toString === "function",
    "cumulative should be BN"
  );
  assert(Array.isArray(anchorFormat.proof), "proof should be array");

  console.log("  ✓ Anchor format conversion works");
}

function testDeterministicRoot() {
  console.log("\nTesting deterministic root...");

  const holders = {
    [generateTestAddress(1)]: "1000",
    [generateTestAddress(2)]: "500",
    [generateTestAddress(3)]: "2500",
  };

  const entries1 = buildHolderEntries(holders);
  const tree1 = new MerkleTree(entries1);

  const entries2 = buildHolderEntries(holders);
  const tree2 = new MerkleTree(entries2);

  assert(
    tree1.getRoot().equals(tree2.getRoot()),
    "Same holder data should produce same root"
  );

  console.log("  ✓ Deterministic root verified");
}

function testLeafHashFormat() {
  console.log("\nTesting leaf hash format (for on-chain matching)...");

  const buf = Buffer.alloc(32, 0);
  buf.writeUInt32BE(1, 28);
  const address = new PublicKey(buf);
  const entry = {
    address,
    balance: BigInt(1000),
    prevCumulative: BigInt(0),
    cumulative: BigInt(1000),
  };

  const leafHash = computeLeafHash(entry);
  assert(leafHash.length === 32, "Leaf hash should be 32 bytes");

  console.log("  Leaf hash:", leafHash.toString("hex"));
  console.log("  Address bytes:", address.toBuffer().toString("hex"));
  console.log("  ✓ Leaf hash computed (verify against on-chain)");
}

function testLargeDataset() {
  console.log("\nTesting with larger dataset (1000 holders)...");

  // Generate 1000 random holders
  const holders: Record<string, string> = {};
  for (let i = 0; i < 1000; i++) {
    // Create deterministic addresses for testing
    const buf = Buffer.alloc(32, 0);
    buf.writeUInt32BE(i, 28);
    const address = new PublicKey(buf).toBase58();
    holders[address] = String(Math.floor(Math.random() * 10000) + 1);
  }

  const startBuild = Date.now();
  const entries = buildHolderEntries(holders);
  const tree = new MerkleTree(entries);
  const buildTime = Date.now() - startBuild;

  console.log(`  ✓ Built tree for 1000 holders in ${buildTime}ms`);

  // Test proof generation
  const startProof = Date.now();
  const vrf = Buffer.alloc(32);
  for (let i = 0; i < 32; i++) vrf[i] = Math.floor(Math.random() * 256);
  const winnerProof = tree.generateWinnerProof(vrf);
  const proofTime = Date.now() - startProof;

  console.log(`  ✓ Generated winner proof in ${proofTime}ms`);
  console.log(`  Proof length: ${winnerProof.proof.length} (expected ~10 for 1000 items)`);

  // Verify the proof
  const verified = tree.verifyProof(
    {
      address: winnerProof.address,
      balance: winnerProof.balance,
      prevCumulative: winnerProof.prevCumulative,
      cumulative: winnerProof.cumulative,
    },
    winnerProof.proof
  );
  assert(verified, "Generated proof should be valid");
  console.log("  ✓ Proof verification passed");
}

// Run all tests
async function runTests() {
  console.log("=== Merkle Tree Tests ===\n");

  try {
    testBuildHolderEntries();
    testMerkleTree();
    testFindWinner();
    testProofFormat();
    testDeterministicRoot();
    testLeafHashFormat();
    testLargeDataset();

    console.log("\n=== All tests passed! ===");
  } catch (error) {
    console.error("\n=== Test failed! ===");
    console.error(error);
    process.exit(1);
  }
}

runTests();
