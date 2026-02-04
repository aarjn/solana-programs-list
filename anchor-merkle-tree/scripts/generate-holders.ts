#!/usr/bin/env ts-node

import { Keypair } from "@solana/web3.js";
import * as fs from "fs";

const count = parseInt(process.argv[2]) || 10000;
const outputFile = process.argv[3] || "holders.json";

console.log(`Generating ${count} holder entries...`);

const holders: Record<string, string> = {};

for (let i = 0; i < count; i++) {
  const address = Keypair.generate().publicKey.toBase58();
  // Random balance between 1 and 1,000,000 (simulating token amounts)
  const balance = Math.floor(Math.random() * 1_000_000) + 1;
  holders[address] = balance.toString();
}

fs.writeFileSync(outputFile, JSON.stringify(holders, null, 2));
console.log(`Generated ${count} holders -> ${outputFile}`);
