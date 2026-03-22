import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Coinflip } from "../target/types/coinflip";
import { expect } from "chai";
import {
  Queue,
  Randomness,
  SB_ON_DEMAND_PID,
  OnDemandProgram,
  sleep,
} from "@switchboard-xyz/on-demand";

const web3 = anchor.web3;

describe("sb-randomness-coinflip", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  const program = anchor.workspace.Coinflip as Program<Coinflip>;

  // Switchboard setup
  const sbQueue = new web3.PublicKey(
    "3XXtmiUzi16kNFNgg2AErEx55ZUn3SB5DsmzVBiNQttE"
  );
  let sbProgram: OnDemandProgram;
  let queue: Queue;
  
  before(async () => {
    sbProgram = await OnDemandProgram.load(SB_ON_DEMAND_PID, provider);
    queue = new Queue(sbProgram, sbQueue);
  });

  it("Initializes player account", async () => {
    const { playerPda } = await getPlayerPda(program.programId, user);
    await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
        player: playerPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const playerData = await program.account.playerAccount.fetch(playerPda);
    expect(playerData.wager.toNumber()).equals(100);
    expect(playerData.allowedUser.toBase58()).equal(
      user.publicKey.toBase58()
    );
  });

  // it("Flips the coin", async () => {
  //   const { playerPda } = await getPlayerPda(program.programId, user);
  //   const { escrowPda } = await getEscrowPda(program.programId);
  
  //   // 1. Create randomness account
  //   const rngKp = web3.Keypair.generate();
  //   const [randomness, ix] = await Randomness.create(sbProgram, rngKp, sbQueue);
  //   const tx = new web3.Transaction().add(ix);
  //   await provider.sendAndConfirm(tx, [rngKp]);
  
  //   // 2. Call your coin_flip instruction
  //   const guess = true; // heads
  //   await program.methods
  //     .coinFlip(randomness.pubkey, guess) // sb_account arg
  //     .accounts({
  //       user: user.publicKey,
  //       player: playerPda,
  //       randomnessAccountData: randomness.pubkey, // must match randomness pubkey
  //       escrowAccount: escrowPda,
  //       systemProgram: web3.SystemProgram.programId,
  //     })
  //     .signers([user])
  //     .rpc();
  
  //   // 3. Wait for Switchboard fulfillment (testnet/mainnet only)
  //   await sleep(5000);
  
  //   // 4. Check randomness result + player state
  //   const randData = await randomness.loadData();
  //   console.log("Randomness result:", randData.result.toString());
  
  //   const playerData = await program.account.playerAccount.fetch(playerPda);
  //   console.log("Player balance after flip:", playerData.wager.toNumber());
  // });
  
});

// === Helpers ===
const getPlayerPda = async (programID, user) => {
  const [playerPda, playerBump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("playerState"), user.publicKey.toBuffer()],
    programID
  );
  return { playerPda, playerBump };
};

const getEscrowPda = async (programID)=>{
  const [escrowPda, escrowBump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("stateEscrow")],
    programID
  );
  return { escrowPda, escrowBump };
}
async function airdrop(connection: any, address: any, amount = 5e9) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}
