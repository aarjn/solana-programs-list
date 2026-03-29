import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorNftMetaplex } from "../target/types/anchor_nft_metaplex";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

const MPL_CORE_PROGRAM_ID = new anchor.web3.PublicKey(
  "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
);

describe("anchor-nft-metaplex", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .AnchorNftMetaplex as Program<AnchorNftMetaplex>;
  const wallet = provider.wallet as anchor.Wallet;

  let collection: Keypair;
  let asset: Keypair;
  let recipient: Keypair;

  before(async () => {
    recipient = Keypair.generate();
    const airdrop = await provider.connection.requestAirdrop(
      recipient.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL,
    );
    await provider.connection.confirmTransaction(airdrop);
  });

  describe("Create Collection", () => {
    it("creates a collection", async () => {
      collection = Keypair.generate();

      const tx = await program.methods
        .createCollection({
          name: "Test Collection",
          uri: "https://example.com/collection.json",
        })
        .accountsPartial({
          collection: collection.publicKey,
          updateAuthority: wallet.publicKey,
          payer: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .signers([collection])
        .rpc();

      console.log("Collection created:", tx);

      const collectionAccount = await provider.connection.getAccountInfo(
        collection.publicKey,
      );
      expect(collectionAccount).to.not.be.null;
    });
  });

  describe("Create Asset", () => {
    it("creates an asset in the collection", async () => {
      asset = Keypair.generate();

      const tx = await program.methods
        .createAsset({
          name: "Test NFT",
          uri: "https://example.com/nft.json",
        })
        .accountsPartial({
          asset: asset.publicKey,
          collection: collection.publicKey,
          authority: wallet.publicKey,
          payer: wallet.publicKey,
          owner: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .signers([asset])
        .rpc();

      console.log("Asset created:", tx);

      const assetAccount = await provider.connection.getAccountInfo(
        asset.publicKey,
      );
      expect(assetAccount).to.not.be.null;
    });
  });

  describe("Transfer Asset", () => {
    it("transfers asset to another wallet", async () => {
      const tx = await program.methods
        .transferAsset()
        .accountsPartial({
          asset: asset.publicKey,
          collection: collection.publicKey,
          payer: wallet.publicKey,
          authority: wallet.publicKey,
          newOwner: recipient.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .rpc();

      console.log("Asset transferred:", tx);

      const assetAccount = await provider.connection.getAccountInfo(
        asset.publicKey,
      );
      expect(assetAccount).to.not.be.null;
    });
  });

  describe("Update Asset", () => {
    it("updates asset metadata", async () => {
      const transferBackTx = await program.methods
        .transferAsset()
        .accountsPartial({
          asset: asset.publicKey,
          collection: collection.publicKey,
          payer: recipient.publicKey,
          authority: recipient.publicKey,
          newOwner: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .signers([recipient])
        .rpc();

      console.log("Transferred back:", transferBackTx);

      const tx = await program.methods
        .updateAsset({
          newName: "Updated NFT",
          newUri: "https://example.com/updated.json",
        })
        .accountsPartial({
          asset: asset.publicKey,
          collection: collection.publicKey,
          payer: wallet.publicKey,
          authority: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .rpc();

      console.log("Asset updated:", tx);

      const assetAccount = await provider.connection.getAccountInfo(
        asset.publicKey,
      );
      expect(assetAccount).to.not.be.null;
    });
  });

  describe("Burn Asset", () => {
    it("burns the asset", async () => {
      const tx = await program.methods
        .burnAsset()
        .accountsPartial({
          asset: asset.publicKey,
          collection: collection.publicKey,
          payer: wallet.publicKey,
          authority: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .rpc();

      console.log("Asset burned:", tx);

      const assetAccount = await provider.connection.getAccountInfo(
        asset.publicKey,
      );
      expect(assetAccount.data.length).to.equal(1);
      expect(assetAccount.data[0]).to.equal(0);
    });
  });

  describe("Full Flow", () => {
    it("runs complete flow: collection -> asset -> transfer -> update -> burn", async () => {
      const newCollection = Keypair.generate();
      const newAsset = Keypair.generate();
      const newRecipient = Keypair.generate();

      const airdrop = await provider.connection.requestAirdrop(
        newRecipient.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL,
      );
      await provider.connection.confirmTransaction(airdrop);

      await program.methods
        .createCollection({
          name: "Flow Collection",
          uri: "https://example.com/flow-collection.json",
        })
        .accountsPartial({
          collection: newCollection.publicKey,
          updateAuthority: wallet.publicKey,
          payer: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .signers([newCollection])
        .rpc();

      console.log("Collection created");

      await program.methods
        .createAsset({
          name: "Flow NFT",
          uri: "https://example.com/flow-nft.json",
        })
        .accountsPartial({
          asset: newAsset.publicKey,
          collection: newCollection.publicKey,
          authority: wallet.publicKey,
          payer: wallet.publicKey,
          owner: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .signers([newAsset])
        .rpc();

      console.log("Asset created");

      await program.methods
        .transferAsset()
        .accountsPartial({
          asset: newAsset.publicKey,
          collection: newCollection.publicKey,
          payer: wallet.publicKey,
          authority: wallet.publicKey,
          newOwner: newRecipient.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .rpc();

      console.log("Asset transferred");

      await program.methods
        .transferAsset()
        .accountsPartial({
          asset: newAsset.publicKey,
          collection: newCollection.publicKey,
          payer: newRecipient.publicKey,
          authority: newRecipient.publicKey,
          newOwner: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .signers([newRecipient])
        .rpc();

      console.log("Asset transferred back");

      await program.methods
        .updateAsset({
          newName: "Updated FLow NFT",
          newUri: "https://example.com/updated-flow.json",
        })
        .accountsPartial({
          asset: newAsset.publicKey,
          collection: newCollection.publicKey,
          payer: wallet.publicKey,
          authority: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .rpc();

      console.log("Asset updated");

      await program.methods
        .burnAsset()
        .accountsPartial({
          asset: newAsset.publicKey,
          collection: newCollection.publicKey,
          payer: wallet.publicKey,
          authority: wallet.publicKey,
          systemProgram: SystemProgram.programId,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        })
        .rpc();

      console.log("Asset burned");

      const assetAccount = await provider.connection.getAccountInfo(
        newAsset.publicKey,
      );
      expect(assetAccount.data.length).to.equal(1);
      expect(assetAccount.data[0]).to.equal(0);

      console.log("🎉 Complete flow test passed!");
    });
  });
});
