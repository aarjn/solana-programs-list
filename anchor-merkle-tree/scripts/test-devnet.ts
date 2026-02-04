#!/usr/bin/env ts-node

/**
 * Full devnet test of the Merkle Tree Weighted Lottery
 */

import * as fs from "fs";
import * as path from "path";
import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, BN } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import {
  AnchorUtils,
  Queue,
  Randomness,
  ON_DEMAND_DEVNET_PID,
  ON_DEMAND_DEVNET_QUEUE,
  asV0Tx,
} from "@switchboard-xyz/on-demand";
import {
  MerkleTree,
  buildHolderEntries,
  proofToAnchorFormat,
  computeSelectionPoint,
} from "./merkle";
import type { AnchorMerkleTree } from "../target/types/anchor_merkle_tree";

const PROGRAM_ID = new PublicKey("HSBQg9YYMu8DtD1pgTfwxTqTdGWTKHtxSjg5wT3bz1mi");

function loadIdl(): any {
  return JSON.parse(
    fs.readFileSync(path.join(__dirname, "../target/idl/anchor_merkle_tree.json"), "utf8")
  );
}

function getLotteryPda(authority: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("lottery"), authority.toBuffer()],
    PROGRAM_ID
  );
}

async function main() {
  console.log("=== Merkle Tree Weighted Lottery - Devnet Test ===\n");

  // Setup
  const { provider, wallet } = await AnchorUtils.loadEnv();
  const payer = wallet.payer;
  const connection = provider.connection;

  console.log("Payer:", payer.publicKey.toBase58());
  console.log("RPC:", connection.rpcEndpoint);

  const IDL = loadIdl();
  const program = new Program(IDL, provider) as Program<AnchorMerkleTree>;

  // Switchboard setup
  const sbProgram = await anchor.Program.at(ON_DEMAND_DEVNET_PID, provider);
  const queueAccount = new Queue(sbProgram as any, ON_DEMAND_DEVNET_QUEUE);

  try {
    await queueAccount.loadData();
  } catch (e) {
    console.error("Failed to load Switchboard queue. Is devnet available?");
    return;
  }

  const [lotteryPda] = getLotteryPda(payer.publicKey);
  console.log("Lottery PDA:", lotteryPda.toBase58());

  // Check if lottery already exists
  let existingLottery: any = null;
  try {
    existingLottery = await program.account.lottery.fetch(lotteryPda);
    console.log("\nExisting lottery found!");
    console.log("  Finalized:", existingLottery.finalized);
    console.log("  VRF set:", !Buffer.from(existingLottery.vrfResult).equals(Buffer.alloc(32, 0)));

    if (existingLottery.finalized) {
      console.log("\n⚠️  Lottery is already finalized. Winner:", existingLottery.winner?.toBase58());
      console.log("To test again, you need to close this lottery or use a different authority.");
      return;
    }
  } catch (e) {
    console.log("\nNo existing lottery found, will create new one.");
  }

  // Step 1: Create holder data
  console.log("\n--- Step 1: Create Holder Data ---");
  const holders: Record<string, string> = {
    [payer.publicKey.toBase58()]: "10000", // Equal weight (1% chance with 100 holders)
  };
  // Add 99 more random holders with similar weights
  for (let i = 0; i < 99; i++) {
    holders[Keypair.generate().publicKey.toBase58()] = String(
      Math.floor(Math.random() * 10000) + 5000  // 5000-15000 each
    );
  }
  console.log("Created 100 holders (payer has ~1% weight, fair lottery)");

  // Step 2: Build Merkle tree
  console.log("\n--- Step 2: Build Merkle Tree ---");
  const entries = buildHolderEntries(holders);
  const tree = new MerkleTree(entries);
  const totalWeight = tree.getTotalWeight();
  const merkleRoot = tree.getRootArray();

  console.log("Total weight:", totalWeight.toString());
  console.log("Merkle root:", tree.getRoot().toString("hex").slice(0, 32) + "...");

  // Step 3: Initialize lottery (if not exists)
  if (!existingLottery) {
    console.log("\n--- Step 3: Initialize Lottery ---");
    const initTx = await program.methods
      .initializeLottery(merkleRoot as number[], new BN(totalWeight.toString()))
      .accountsStrict({
        lottery: lotteryPda,
        authority: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();
    console.log("Lottery initialized:", initTx);
  } else {
    console.log("\n--- Step 3: Using Existing Lottery ---");
    console.log("Note: Merkle root was set during previous init");
  }

  // Step 4: Create Switchboard randomness
  console.log("\n--- Step 4: Create Switchboard Randomness ---");
  const rngKp = Keypair.generate();
  console.log("Randomness account:", rngKp.publicKey.toBase58());

  const [randomness, createIx] = await Randomness.create(
    sbProgram as any,
    rngKp,
    ON_DEMAND_DEVNET_QUEUE,
    payer.publicKey
  );

  const createTx = await asV0Tx({
    connection,
    ixs: [createIx],
    payer: payer.publicKey,
    signers: [payer, rngKp],
    computeUnitPrice: 75_000,
    computeUnitLimitMultiple: 1.3,
  });
  await provider.sendAndConfirm(createTx);
  console.log("Randomness account created ✅");

  // Step 5: Commit randomness
  console.log("\n--- Step 5: Commit Randomness ---");
  const commitIx = await randomness.commitIx(ON_DEMAND_DEVNET_QUEUE);
  const commitTx = await asV0Tx({
    connection,
    ixs: [commitIx],
    payer: payer.publicKey,
    signers: [payer],
    computeUnitPrice: 75_000,
    computeUnitLimitMultiple: 1.3,
  });
  await provider.sendAndConfirm(commitTx);
  console.log("Randomness committed ✅");

  // Step 6: Reveal randomness AND set VRF in same transaction
  // (Switchboard randomness expires quickly, must use immediately)
  console.log("\n--- Step 6: Reveal Randomness + Set VRF Result ---");
  const revealIx = await randomness.revealIx();
  const setVrfIx = await program.methods
    .setVrfResult()
    .accountsStrict({
      lottery: lotteryPda,
      randomnessAccount: randomness.pubkey,
      authority: payer.publicKey,
    })
    .instruction();

  const revealAndSetTx = await asV0Tx({
    connection,
    ixs: [revealIx, setVrfIx],
    payer: payer.publicKey,
    signers: [payer],
    computeUnitPrice: 75_000,
    computeUnitLimitMultiple: 1.5,
  });
  await provider.sendAndConfirm(revealAndSetTx);
  console.log("Randomness revealed + VRF result set ✅");

  // Fetch VRF result
  const lottery = await program.account.lottery.fetch(lotteryPda);
  const vrfResult = Buffer.from(lottery.vrfResult);
  console.log("VRF result:", vrfResult.toString("hex"));

  // Use the on-chain merkle root's corresponding tree for winner finding
  // For existing lottery, we need to use its original holders
  // For new lottery, we use our holders

  // Step 7: Find winner off-chain
  console.log("\n--- Step 7: Find Winner (Off-chain) ---");
  const onChainTotalWeight = BigInt(lottery.totalWeight.toString());
  const selectionPoint = computeSelectionPoint(vrfResult, onChainTotalWeight);

  console.log("On-chain total weight:", onChainTotalWeight.toString());
  console.log("Selection point:", selectionPoint.toString());

  // Find winner from our tree (only works if we just initialized)
  if (!existingLottery) {
    const { entry: winner, index } = tree.findWinner(vrfResult);
    console.log("Winner index:", index);
    console.log("Winner address:", winner.address.toBase58());
    console.log("Winner balance:", winner.balance.toString());
    console.log("Winner range: [", winner.prevCumulative.toString(), ",", winner.cumulative.toString(), ")");

    const isPayerWinner = winner.address.equals(payer.publicKey);
    console.log("\nIs payer the winner?", isPayerWinner ? "YES ✅" : "NO ❌");

    if (!isPayerWinner) {
      console.log("\nPayer didn't win this round. Only the winner can claim.");
      console.log("The lottery is ready - winner needs to claim with their proof.");
      console.log("\nTo verify: run this script again to see the same winner (deterministic)");
      return;
    }

    // Step 8: Claim winner
    console.log("\n--- Step 8: Claim Winner (On-chain Verification) ---");
    const winnerProof = tree.generateWinnerProof(vrfResult);
    const anchorProof = proofToAnchorFormat(winnerProof);

    console.log("Submitting claim with proof length:", anchorProof.proof.length);

    const claimTx = await program.methods
      .claimWinner({
        address: anchorProof.address,
        balance: anchorProof.balance,
        prevCumulative: anchorProof.prevCumulative,
        cumulative: anchorProof.cumulative,
        proof: anchorProof.proof,
      })
      .accountsStrict({
        lottery: lotteryPda,
        claimant: payer.publicKey,
      })
      .signers([payer])
      .rpc();
    console.log("Winner claimed:", claimTx);

    // Step 9: Verify final state
    console.log("\n--- Step 9: Final State ---");
    const finalLottery = await program.account.lottery.fetch(lotteryPda);
    console.log("Finalized:", finalLottery.finalized);
    console.log("Winner on-chain:", finalLottery.winner?.toBase58());
    console.log("Winner weight:", finalLottery.winnerWeight.toString());

    console.log("\n=== TEST COMPLETE ✅ ===");
  } else {
    console.log("\n⚠️  Cannot find winner for existing lottery (don't have original holder data)");
    console.log("VRF has been updated. To test claim, re-run with fresh lottery.");
  }

  console.log("\nExplorer: https://explorer.solana.com/address/" + lotteryPda.toBase58() + "?cluster=devnet");
}

main().catch(console.error);
