# Anchor NFT Metaplex Program

A streamlined Solana program built with the **Anchor Framework** to handle NFT minting and metadata creation using the **Metaplex** standard.

## Features

- **MINTING**: Fully functional NFT minting logic.
- **METAPLEX INTEGRATION**: Automatically handles Metadata and Master Edition account creation.

## Development & Testing

We used **Surfpool** to manage our testing environment and handle RPC interactions, ensuring smooth deployment even when public nodes are congested.

### Prerequisites

- Rust
- Solana CLI
- Anchor CLI 0.32.1
- [Surfpool](https://www.surfpool.run/)

### Local Testing

To run the test suite locally:

```bash
surfpool start
```

```bash
anchor test
```

### Deployment

If you encounter "Rate Limit" errors or connectivity issues with the default Solana Mainnet RPC, use a dedicated provider like **Helius** via the Surfpool command:

```bash
surfpool start --rpc-url "https://mainnet.helius-rpc.com/?api-key=HELIUS_API_KEY"
```

> **Note:** If the transaction fails due to network congestion, consistently try switching to a different RPC provider or a private endpoint as shown above.

## Test Results

> <img width="1653" height="769" alt="Screenshot from 2026-03-24 10-11-55" src="https://github.com/user-attachments/assets/4493144d-b311-4317-aea4-dd9cc162a509" />

## Dependencies

This program specifically utilizes:

- anchor-lang = "0.32.1"
- mpl-token-metadata = "5.1.1"

## Acknowledgments

- **Metaplex Foundation**: For the Token Metadata standard.
- **Surfpool**: For providing the specialized CLI tools used during the testing and deployment phase of this project.
