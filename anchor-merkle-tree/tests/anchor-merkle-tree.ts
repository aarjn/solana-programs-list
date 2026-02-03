import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import {
  AnchorUtils,
  Queue,
  Randomness,
  ON_DEMAND_DEVNET_PID,
  ON_DEMAND_DEVNET_QUEUE,
  asV0Tx,
} from "@switchboard-xyz/on-demand";
import { AnchorMerkleTree } from "../target/types/anchor_merkle_tree";

describe("Switchboard Randomness Example", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .AnchorMerkleTree as Program<AnchorMerkleTree>;
  const provider = anchor.AnchorProvider.env();

  // Switchboard setup
  const sbQueue = ON_DEMAND_DEVNET_QUEUE;
  const sbProgramId = ON_DEMAND_DEVNET_PID;

  let sbProgram: Program;
  let queueAccount: Queue;
  let payer: Keypair;

  before(async () => {
    const { provider, wallet } = await AnchorUtils.loadEnv();
    payer = wallet.payer;
    sbProgram = await anchor.Program.at(sbProgramId, provider);
    // @ts-ignore
    queueAccount = new Queue(sbProgram, sbQueue);

    console.log("Payer:", payer.publicKey.toString());
    console.log("Program ID:", program.programId.toString());
    console.log("Switchboard Devnet Program ID:", sbProgramId.toString());
    console.log("Switchboard Devnet Queue:", sbQueue.toString());
    console.log("--------------------------------");
  });

  it("Logs a random value", async () => {
    try {
      // Generate a random keypair for the randomness account
      const rngKp = Keypair.generate();
      console.log("Creating randomness account:", rngKp.publicKey.toString());

      try {
        await queueAccount.loadData();
      } catch (error) {
        console.log("error:", error);
        return;
      }
      // Create the randomness account and get the creation instruction
      const [randomness, createIx] = await Randomness.create(
        sbProgram as any,
        rngKp,
        sbQueue,
        payer.publicKey
      );

      const createRandomnessTx = await asV0Tx({
        connection: provider.connection,
        ixs: [createIx],
        payer: payer.publicKey,
        signers: [payer, rngKp],
        computeUnitPrice: 75_000,
        computeUnitLimitMultiple: 1.3,
      });
      const createRandomnessTxHash = await provider.sendAndConfirm(
        createRandomnessTx
      );
      console.log("--------------------------------");
      console.log("Randomness account created ✅");
      console.log(
        "Create randomness transaction hash:",
        createRandomnessTxHash
      );

      const commitRandomnessIx = await program.methods
        .commitRandomness()
        .accounts({
          randomnessAccount: randomness.pubkey,
        })
        .instruction();
      const sbCommitIx = await randomness.commitIx(sbQueue);
      const commitTx = await asV0Tx({
        connection: provider.connection,
        ixs: [sbCommitIx, commitRandomnessIx],
        payer: payer.publicKey,
        signers: [payer],
        computeUnitPrice: 75_000,
        computeUnitLimitMultiple: 1.3,
      });
      const commitTxHash = await provider.sendAndConfirm(commitTx);
      console.log("--------------------------------");
      console.log("Commit randomness transaction hash:", commitTxHash);

      const sbRevealIx = await randomness.revealIx();
      const revealRandomnessIx = await program.methods
        .revealAndLogRandom()
        .accounts({
          randomnessAccount: randomness.pubkey,
        })
        .instruction();
      const revealIx = await asV0Tx({
        connection: provider.connection,
        ixs: [sbRevealIx, revealRandomnessIx],
        payer: payer.publicKey,
        signers: [payer],
        computeUnitPrice: 75_000,
        computeUnitLimitMultiple: 1.3,
      });
      const revealTxHash = await provider.sendAndConfirm(revealIx);
      console.log("--------------------------------");
      console.log("Reveal randomness transaction hash:", revealTxHash);
    } catch (error) {
      console.error("Error in test:", error);
    }
  });
});
