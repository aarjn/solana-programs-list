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

## Contents

- [📚 Core Concepts](#-core-concepts)
- [🏦 DeFi](#-defi)
- [🖼️ NFTs & Tokens](#%EF%B8%8F-nfts--tokens)
- [🌳 Data Structures](#-data-structures)
- [🕹️ Games](#%EF%B8%8F-games)
- [🔐 Privacy & Security](#-privacy--security)

**Legend:** 🟢 Completed | 🟡 In Progress | 🔴 Planned | 🏗️ WIP | ✅ Tests | ❌ No Tests

## 📚 Core Concepts

| Program | Description | ⚓ Anchor | 🦀 Native | 🤥 Pinocchio | ✨ Quasar |
|---------|-------------|-----------|-----------|-------------|----------|
| Hello World | Hello World | - | - | [🤥](https://github.com/4rjunc/solana-programs-list/tree/main/pinocchio-hello-world) | [✨](https://github.com/4rjunc/solana-programs-list/tree/main/quasar-hello-world) |
| Counter | Simple counter app | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-counterapp) 🟢 ✅ | [🦀](https://github.com/4rjunc/solana-programs-list/tree/main/native-counter) 🟢 | [🤥](https://github.com/4rjunc/solana-programs-list/tree/main/pinocchio-counterapp) 🏗️ | [✨](https://github.com/4rjunc/solana-programs-list/tree/main/quasar-counter) 🟢 ✅ |
| PDA Demo | Program Derived Addresses & CRUD | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-pda) 🟢 ✅ / [CRUD](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-pda-crud) 🟢 ✅ | [🦀](https://github.com/4rjunc/solana-programs-list/tree/main/native-pda) 🟢 / [CRUD](https://github.com/4rjunc/solana-programs-list/tree/main/native-pda-crud) 🟢 ✅ | - | - |
| CPI | Cross-Program Invocations | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-cpi) 🟢 ✅ | [🦀](https://github.com/4rjunc/solana-programs-list/tree/main/native-cpi-basic) 🟢 ✅ | - | - |
| Account Checks | Account validation patterns | - | [🦀](https://github.com/4rjunc/solana-programs-list/tree/main/native-account-checks) 🟢 ✅ | - | - |
| Account Data | On-chain data records & serialization | - | [🦀](https://github.com/4rjunc/solana-programs-list/tree/main/native-account-data) 🟢 ✅ | - | - |

## 🏦 DeFi

| Program | Description | ⚓ Anchor | 🦀 Native | 🤥 Pinocchio | ✨ Quasar |
|---------|-------------|-----------|-----------|-------------|----------|
| Sol Vault | Deposit and withdraw SOL | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-sol-vault) 🟢 ✅ / [Manager](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-vault-manager) 🟢 ✅ / [Blueshift](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-vault-blueshift) 🟢 | - | [🤥](https://github.com/4rjunc/pinocchio-vault/) / [Blueshift](https://github.com/4rjunc/solana-programs-list/tree/main/pinocchio-blueshift_vault) 🟢 | [✨](https://github.com/Vinayapr23/quasar-vault/tree/b6696c3e6f6db4cbcce1875ce68179c879333531) 🟢 ✅ |
| Escrow | Secure token swaps | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-escrow) 🟢 ✅ / [Blueshift](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-escrow-blueshift) 🟢 ✅ | [🦀](https://github.com/4rjunc/solana-programs-list/tree/main/native-escrow) 🟢 ✅ | [🤥](https://github.com/4rjunc/solana-programs-list/tree/main/pinocchio-escrow) 🟡 | [✨](https://github.com/4rjunc/solana-programs-list/tree/main/quasar-escrow) 🟢 |
| AMM | Classic Automated Market Maker | [⚓](https://github.com/subhdotsol/AMM-Sandbox) 🟢 | - | [🤥](https://github.com/AvhiMaz/pinocchio-amm) | - |
| Bonding Curve | Simple bonding curve | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-bonding-curve) 🟡 | - | - | - |
| Lending | Lend token/assets | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-lending-protocol) 🏗️ | - | - | - |
| Collateral Stablecoin | Collateral-backed stablecoin with oracle & liquidation | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-collateral-stablecoin) 🟢 ✅ | - | - | - |
| Staking | Stake assets for rewards | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-staking) 🟡 | - | - | - |

## 🖼️ NFTs & Tokens

| Program | Description | ⚓ Anchor | 🦀 Native | 🤥 Pinocchio | ✨ Quasar |
|---------|-------------|-----------|-----------|-------------|----------|
| SPL Token | SPL token mint, transfer & accounts | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-spl-token) 🟢 ✅ | - | - | - |
| Token Creation | Create tokens with metadata | - | [🦀](https://github.com/4rjunc/solana-programs-list/tree/main/native-create-token) 🟢 ✅ | - | - |
| NFT Minting | Create & manage NFT collections | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-nft-metaplex) 🟡 | - | - | - |
| pNFT | Programmable NFT implementation | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-p-nft) 🔴 | - | - | - |

## 🌳 Data Structures

| Program | Description | ⚓ Anchor | 🦀 Native | 🤥 Pinocchio | ✨ Quasar |
|---------|-------------|-----------|-----------|-------------|----------|
| Merkle Tree | Incremental Merkle tree with insert & proof verification | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-merkle-tree-incremental) 🟢 ✅ | - | - | [✨](https://github.com/4rjunc/solana-programs-list/tree/main/quasar-merkle-tree) 🟢 ✅ |
| Merkle Tree Lottery | Merkle tree lottery with VRF randomness | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-merkle-tree) 🟢 ✅ | - | - | - |

## 🕹️ Games

| Program | Description | ⚓ Anchor | 🦀 Native | 🤥 Pinocchio | ✨ Quasar |
|---------|-------------|-----------|-----------|-------------|----------|
| Tic Tac Toe | On-chain Tic Tac Toe | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-tic-tac-toe) 🟡 | - | - | - |

## 🔐 Privacy & Security

| Program | Description | ⚓ Anchor | 🦀 Native | 🤥 Pinocchio | ✨ Quasar |
|---------|-------------|-----------|-----------|-------------|----------|
| Arcium Hello World | Arcium confidential computing demo | [⚓](https://github.com/4rjunc/solana-programs-list/tree/main/anchor-arcium-hello-world) 🟡 | - | - | - |
| Multi Sign | Multi-signature transaction signing | [⚓](https://github.com/4rjunc/solana-dual-signing/) 🟢 ✅ | - | - | - |




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
