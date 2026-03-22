import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dicepool } from "../target/types/dicepool";
import { expect } from "chai";

const web3 = anchor.web3;

describe("dicepool", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Dicepool as Program<Dicepool>;
  const user = (provider.wallet as anchor.Wallet).payer;

  const users = Array.from({ length: 8 }, () =>
    anchor.web3.Keypair.generate()
  );

  const activePoolId = new anchor.BN(1);
  const endedPoolId = new anchor.BN(2);

  // 🔧 Helpers
  const getPoolPda = (programID, user, poolId) => {
    return web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("dice_pool"),
        user.publicKey.toBuffer(),
        poolId.toArrayLike(Buffer, "le", 8),
      ],
      programID
    );
  };

  const getPlayerPda = (programID, user, poolId) => {
    return web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("dice_player"),
        user.toBuffer(),
        poolId.toArrayLike(Buffer, "le", 8),
      ],
      programID
    );
  };

  async function airdrop(connection, address, amount = 10 * web3.LAMPORTS_PER_SOL) {
    const sig = await connection.requestAirdrop(address, amount);
    await connection.confirmTransaction(sig, "confirmed");
  }

  // 🏗️ Create ACTIVE pool (for betting)
  it("Create Active Pool", async () => {
    const startTime = new anchor.BN(Math.floor(Date.now() / 1000) - 3600);
    const endTime = new anchor.BN(Math.floor(Date.now() / 1000) + 7200);

    const capacity = new anchor.BN(10);
    const baseAmount = new anchor.BN(50000000);

    const [poolPda] = getPoolPda(program.programId, user, activePoolId);

    await program.methods
      .createPool(activePoolId, startTime, endTime, capacity, baseAmount)
      .accounts({
        payer: user.publicKey,
        dicePool: poolPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const pool = await program.account.dicePool.fetch(poolPda);
    expect(pool.id.toNumber()).to.equal(activePoolId.toNumber());
  });

  // 🎲 Betting
  it("Users should Bet in Pool", async () => {
    const [poolPda] = getPoolPda(program.programId, user, activePoolId);

    for (let i = 0; i < users.length; i++) {
      await airdrop(provider.connection, users[i].publicKey);

      const betAmount = new anchor.BN(50000000);
      const randomNumber = Math.floor(Math.random() * 6) + 1;

      const [playerPda] = getPlayerPda(
        program.programId,
        users[i].publicKey,
        activePoolId
      );

      await program.methods
        .joinPool(activePoolId, betAmount, new anchor.BN(randomNumber))
        .accounts({
          payer: users[i].publicKey,
          dicePool: poolPda,
          dicePlayer: playerPda,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([users[i]])
        .rpc();
    }

    const pool = await program.account.dicePool.fetch(poolPda);

    expect(pool.betters.length).to.equal(8);
    expect(pool.totalAmount.toNumber()).to.equal(400000000);
    expect(pool.ended).to.equal(false);
  });

  // 🏗️ Create ENDED pool (for result test)
  it("Create Ended Pool", async () => {
    const startTime = new anchor.BN(Math.floor(Date.now() / 1000) - 7200);
    const endTime = new anchor.BN(Math.floor(Date.now() / 1000) - 10);

    const capacity = new anchor.BN(10);
    const baseAmount = new anchor.BN(50000000);

    const [poolPda] = getPoolPda(program.programId, user, endedPoolId);

    await program.methods
      .createPool(endedPoolId, startTime, endTime, capacity, baseAmount)
      .accounts({
        payer: user.publicKey,
        dicePool: poolPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const pool = await program.account.dicePool.fetch(poolPda);
    expect(pool.id.toNumber()).to.equal(endedPoolId.toNumber());
  });

  // 🎯 Set Result
  it("Set Result by pool creator", async () => {
    const [poolPda] = getPoolPda(program.programId, user, endedPoolId);

    await program.methods
      .setResult(endedPoolId, new anchor.BN(3), new anchor.BN(100))
      .accounts({
        creator: user.publicKey,
        dicePool: poolPda,
      })
      .signers([user])
      .rpc();

    const pool = await program.account.dicePool.fetch(poolPda);

    expect(pool.ended).to.equal(true);
    expect(pool.result.toNumber()).to.equal(3);
  });
});