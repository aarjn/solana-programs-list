#!/usr/bin/env ts-node

import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { AnchorUtils } from "@switchboard-xyz/on-demand";
import * as fs from "fs";
import * as path from "path";
import type { AnchorMerkleTree } from "../target/types/anchor_merkle_tree";

const PROGRAM_ID = new PublicKey("HSBQg9YYMu8DtD1pgTfwxTqTdGWTKHtxSjg5wT3bz1mi");

async function main() {
  const { provider, wallet } = await AnchorUtils.loadEnv();

  const IDL = JSON.parse(
    fs.readFileSync(path.join(__dirname, "../target/idl/anchor_merkle_tree.json"), "utf8")
  );
  const program = new Program(IDL, provider) as Program<AnchorMerkleTree>;

  const [lotteryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("lottery"), wallet.payer.publicKey.toBuffer()],
    PROGRAM_ID
  );

  console.log("Authority:", wallet.payer.publicKey.toBase58());
  console.log("Lottery PDA:", lotteryPda.toBase58());

  try {
    // Check if lottery exists
    const lottery = await program.account.lottery.fetch(lotteryPda);
    console.log("\nLottery found:");
    console.log("  Finalized:", lottery.finalized);
    console.log("  Winner:", lottery.winner?.toBase58() || "None");

    // Close it
    const tx = await program.methods
      .closeLottery()
      .accountsStrict({
        lottery: lotteryPda,
        authority: wallet.payer.publicKey,
      })
      .signers([wallet.payer])
      .rpc();

    console.log("\n✅ Lottery closed!");
    console.log("Tx:", tx);
  } catch (e: any) {
    if (e.message?.includes("Account does not exist")) {
      console.log("\n❌ No lottery found to close.");
    } else {
      throw e;
    }
  }
}

main().catch(console.error);
