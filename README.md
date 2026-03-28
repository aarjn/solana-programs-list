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

The full categorized program index lives in [PROGRAMS.md](PROGRAMS.md).

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
