<div align="center">
 <img src="./banner.png" alt="solana" width="380">

 <h2> Solana Programs Collection </h2>
 <h4> 30+ Programs Added </h4>
</div>

A curated collection of Solana programs built with Rust

## Repository Structure

Each program is organized in its own dedicated folder with a clear naming convention:

- For Anchor framework programs: `anchor-[programname]`
- For native Solana programs: `native-[programname]`
- For general notation of framework programs: `[framework]-[programname]`

## Programs Included

**Legend:**

- 🟢 Completed
- 🟡 In Progress / Half Done
- 🔴 Planned
- 🏗️ Work in progress
- ✅ Tests Available
- ❌ No Tests

| Program                                                                                     | Description                      | Features                                      | ⚓ Anchor                            | 🦀 Native        |🤥Pinocchio  | ✨ Quasar |
|--------------------------------|--------------------------------|--------------------------------|--------------------------------|--------------------------------|--------------------------------|--------------------------------|
| Hello World     | Hello World               | `Hello World`                      | NIL       |  NIL      | [🤥 Program](https://github.com/4rjunc/solana-programs-list/tree/main/pinocchio-hello-world)  | [✨ Program](https://github.com/4rjunc/solana-programs-list/tree/main/quasar-hello-world) |
| Arcium Hello World     | Arcium confidential computing demo               | `Arcium` `Confidential Computing` `CPI`                      | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-arcium-hello-world) 🟡 ❌       |  NIL      | NIL  | NIL |
| Counterapp  | Simple counter app                | `PDA`                                         | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-counterapp) 🟢 ✅      | [🦀 Program](https://github.com/4rjunc/solana-programs-list/tree/main/native-counter) 🟢 ❌    | [🤥 Program](https://github.com/4rjunc/solana-programs-list/tree/main/pinocchio-counterapp)  🏗️    | NIL |
| NFT Minting | Create & manage NFT collections  | `Metadata` `Metaplex` `Mint` `Transfer` `CPI` | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-nft-metaplex) 🟡       | NIL    | NIL    | NIL |
| pNFT | Programmable NFT implementation  | `pNFT` `Metaplex` `Token Metadata` | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-p-nft) 🔴 ❌       | NIL    | NIL    | NIL |
| Sol Vault   | Deposit and withdraw Sol         | `Deposit` `Withdraw` `PDA`                    | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-sol-vault) 🟢 ✅ & [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-vault-manager) 🟢 ✅ & [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-vault-blueshift) 🟢 ❌   | NIL    | [🤥 Program](https://github.com/4rjunc/pinocchio-vault/) & [🤥 Program](https://github.com/4rjunc/solana-programs-list/tree/main/pinocchio-blueshift_vault) 🟢 ❌    | NIL |
| PDA Demo    | Simple program to demonstrate PDA | `PDA` `CRUD`                                         | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-pda) 🟢 ✅  & [⚓ CRUD Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-pda-crud) 🟢 ✅     | [🦀 Program](https://github.com/4rjunc/solana-programs-list/tree/main/native-pda) 🟢 ❌ & [🦀 CRUD Program](https://github.com/4rjunc/solana-programs-list/tree/main/native-pda-crud) 🟢 ✅    | NIL    | NIL |
| Escrow      | Secure token swaps               | `Lock` `Release` `Cancel`                     | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-escrow) 🟢 ✅ & [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-escrow-blueshift) 🟢 ✅      | [🦀 Program](https://github.com/4rjunc/solana-programs-list/tree/main/native-escrow) 🟢 ✅      | [🤥 Program](https://github.com/4rjunc/solana-programs-list/tree/main/pinocchio-escrow) 🟡 ❌    | NIL |
| Multi Sign      | Signing tx multiple times               | `Signing` `Fullstack`                     | [⚓ Program](https://github.com/4rjunc/solana-dual-signing/) 🟢 ✅       | NIL      | NIL    | NIL |
| Lending      | Lend token/assets               | `Tokens` `Locking` `Lend`                     | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-lending-protocol) 🏗️ ❌       | NIL      | NIL    | NIL |
| Collateral Stablecoin      | Collateral-backed stablecoin protocol               | `Lending` `Collateral` `Oracle` `Liquidation` `Token2022`                     | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-collateral-stablecoin) 🟢 ✅       | NIL      | NIL    | NIL |
| Stake      | Stake assets               | `Tokens` `Stake` `Reward`                     | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-staking) 🟡       | NIL      | NIL    | NIL |
| Tic Tac Toe      | Tic Tac Toe               | `PDA` `Mini Game`                      | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-tic-tac-toe) 🟡       | NIL      | NIL    | NIL |
| SPL Token      | SPL token mint, transfer & accounts               | `SPL Token` `Mint` `Transfer` `ATA`                      | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-spl-token) 🟢 ✅       | NIL      | NIL    | NIL |
| Token Creation      | Create tokens with metadata               | `Token` `Mint` `Metaplex` `Metadata`                      | NIL       | [🦀 Program](https://github.com/4rjunc/solana-programs-list/tree/main/native-create-token) 🟢 ✅      | NIL    | NIL |
| CPI      | Simple Programs on CPIs               | `CPI` `Transfers`                      | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-cpi) 🟢 ✅       |   [🦀 Program](https://github.com/4rjunc/solana-programs-list/tree/main/native-cpi-basic) 🟢 ✅ | NIL  | NIL |
| Account Checks      | Account validation patterns               | `Account Validation` `Checks` `Educational`                      | NIL       | [🦀 Program](https://github.com/4rjunc/solana-programs-list/tree/main/native-account-checks) 🟢 ✅      | NIL    | NIL |
| Account Data      | Create & manage on-chain data records               | `Serialization` `Borsh` `Data Storage` `CPI`                      | NIL       | [🦀 Program](https://github.com/4rjunc/solana-programs-list/tree/main/native-account-data) 🟢 ✅      | NIL    | NIL |
| Merkle Tree Lottery      | Merkle tree lottery with VRF randomness               | `Merkle Tree` `Lottery` `VRF` `Switchboard`                      | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-merkle-tree) 🟢 ✅       | NIL      | NIL    | NIL |
| Bonding Curve      | Simple Bonding Curve                | `Bonding Curve` `Trade`                      | [⚓ Program](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-bonding-curve) 🟡      |   |   | NIL |
| Amm      | Classic AMM                | `AMM`                       | [⚓ Program](https://github.com/subhdotsol/AMM-Sandbox) | NIL  |  [🤥 Program](https://github.com/AvhiMaz/pinocchio-amm)  | NIL |




## Prerequisites

- Solana CLI
- Rust
- Anchor (for Anchor framework programs)
- Node.js (for deployment and testing scripts)

## Getting Started

1. Clone the repository

```bash
git clone https://github.com/4rjunc/solana-programs-list.git
cd solana-programs-list
```

2. Set up your Solana environment
3. Navigate to individual program directories
4. Follow specific program `README.md` instructions

## Building Programs

For Anchor programs:

```bash
anchor build
```

For native Solana programs:

```bash
cargo build-sbf
```

## Testing

Each program includes its own test suite. Refer to individual program documentation for testing instructions.

## Contributing

Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

[Specify your license, e.g., MIT License]
