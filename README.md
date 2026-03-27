<p align="center">
  <img src="banner.png" alt="Solana logo" width="320" />
</p>

<h1 align="center">Solana Programs Collection</h1>

<p align="center">
  A curated collection of Solana programs built with Rust across Anchor, native
  Solana, Pinocchio, and Quasar examples.
</p>

## Repository Structure

Each program is organized in its own dedicated folder with a clear naming convention:

- Anchor framework programs: `anchor-[programname]`
- Native Solana programs: `native-[programname]`
- Other frameworks: `[framework]-[programname]`

## Program Index

Browse the repository by category:

- [Core Concepts](#core-concepts)
- [DeFi](#defi)
- [NFTs and Tokens](#nfts-and-tokens)
- [Data Structures](#data-structures)
- [Games](#games)
- [Privacy and Security](#privacy-and-security)

The grouped index is also available in [PROGRAMS.md](PROGRAMS.md).

### Core Concepts

| Program | Description | Implementations | Features |
| --- | --- | --- | --- |
| Hello World | Minimal hello world examples for Solana programs | [Pinocchio](pinocchio-hello-world), [Quasar](quasar-hello-world) | `Hello World` |
| Counterapp | Simple counter app | [Anchor](anchor-counterapp), [Native](native-counter), [Pinocchio](pinocchio-counterapp), [Quasar](quasar-counter) | `PDA` |
| PDA Demo | Simple programs to demonstrate PDA usage and CRUD flows | [Anchor](anchor-pda), [Anchor CRUD](anchor-pda-crud), [Native](native-pda), [Native CRUD](native-pda-crud) | `PDA` `CRUD` |
| CPI | Simple programs using CPI | [Anchor](anchor-cpi), [Native](native-cpi-basic) | `CPI` `Transfers` |
| Account Data | Create and manage on-chain data records | [Native](native-account-data) | `Serialization` `Borsh` `Data Storage` `CPI` |

### DeFi

| Program | Description | Implementations | Features |
| --- | --- | --- | --- |
| Sol Vault | Deposit and withdraw SOL | [Anchor](anchor-sol-vault), [Anchor Manager](anchor-vault-manager), [Anchor Blueshift](anchor-vault-blueshift), [Pinocchio](pinocchio-vault), [Pinocchio Blueshift](pinocchio-blueshift_vault), [Quasar](quasar-vault) | `Deposit` `Withdraw` `PDA` |
| Escrow | Secure token swaps | [Anchor](anchor-escrow), [Anchor Blueshift](anchor-escrow-blueshift), [Native](native-escrow), [Pinocchio](pinocchio-escrow), [Quasar](quasar-escrow) | `Lock` `Release` `Cancel` |
| Lending | Lend tokens and assets | [Anchor](anchor-lending-protocol) | `Tokens` `Locking` `Lend` |
| Collateral Stablecoin | Collateral-backed stablecoin protocol | [Anchor](anchor-collateral-stablecoin) | `Lending` `Collateral` `Oracle` `Liquidation` `Token2022` |
| Stake | Stake assets | - | `Tokens` `Stake` `Reward` |
| Bonding Curve | Simple bonding curve | - | `Bonding Curve` `Trade` |
| AMM | Classic AMM | [Anchor](anchor-amm), [Pinocchio](pinocchio-amm) | `AMM` |

### NFTs and Tokens

| Program | Description | Implementations | Features |
| --- | --- | --- | --- |
| NFT Minting | Create and manage NFT collections | [Anchor](anchor-nft-metaplex) | `Metadata` `Metaplex` `Mint` `Transfer` `CPI` |
| pNFT | Programmable NFT implementation | [Anchor](anchor-p-nft) | `pNFT` `Metaplex` `Token Metadata` |
| SPL Token | SPL token mint, transfer, and accounts | [Anchor](anchor-spl-token) | `SPL Token` `Mint` `Transfer` `ATA` |
| Token Creation | Create tokens with metadata | [Native](native-create-token) | `Token` `Mint` `Metaplex` `Metadata` |

### Data Structures

| Program | Description | Implementations | Features |
| --- | --- | --- | --- |
| Merkle Tree | Incremental Merkle tree with insert and proof verification | [Anchor](anchor-merkle-tree-incremental), [Quasar](quasar-merkle-tree) | `Merkle Tree` `Hashing` `Proof Verification` |

### Games

| Program | Description | Implementations | Features |
| --- | --- | --- | --- |
| Tic Tac Toe | Tic Tac Toe game program | [Anchor](anchor-tic-tac-toe) | `PDA` `Mini Game` |
| Merkle Tree Lottery | Merkle tree lottery with VRF randomness | [Anchor](anchor-merkle-tree) | `Merkle Tree` `Lottery` `VRF` `Switchboard` |

### Privacy and Security

| Program | Description | Implementations | Features |
| --- | --- | --- | --- |
| Arcium Hello World | Arcium confidential computing demo | [Anchor](anchor-arcium-hello-world) | `Arcium` `Confidential Computing` `CPI` |
| Multi Sign | Signing transactions multiple times | - | `Signing` `Fullstack` |
| Account Checks | Account validation patterns | [Native](native-account-checks) | `Account Validation` `Checks` `Educational` |

## Prerequisites

- Solana CLI
- Rust
- Anchor (for Anchor framework programs)
- Node.js (for deployment and testing scripts)

## Getting Started

1. Clone the repository.

```bash
git clone https://github.com/4rjunc/solana-programs-list.git
cd solana-programs-list
```

2. Set up your Solana environment.
3. Navigate to the program directory you want to explore.
4. Follow the program-specific `README.md` instructions.

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

Each program includes its own test suite. Refer to the relevant program documentation for test commands and setup.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for the contribution process and documentation requirements.

## License

MIT
