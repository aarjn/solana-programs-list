# Quasar Hello World

A minimal Solana program built with [Quasar](https://quasar-lang.com/docs), a high-performance framework for writing SVM programs.

Quasar uses a zero-copy, `no_std` architecture where accounts are pointer-cast directly from the SVM input buffer — eliminating deserialization, heap allocation, and data copying. This results in dramatically lower compute unit (CU) consumption and smaller binary sizes compared to traditional frameworks.

## Program Overview

This program defines a single `initialize` instruction that validates a payer (signer) and the system program. It serves as a starting point for building more complex Quasar programs.

**Program ID:** `4qm3iMXJFdPh8uH3uiWiDj9bBYUHaP5D1FGKQCD87kez`

### Accounts

| Account          | Type              | Description                        |
| ---------------- | ----------------- | ---------------------------------- |
| `payer`          | `&mut Signer`     | Mutable signer funding the transaction |
| `system_program` | `Program<System>` | The Solana System Program          |

### Instructions

| Instruction   | Discriminator | Description                              |
| ------------- | ------------- | ---------------------------------------- |
| `initialize`  | `0`           | Validates accounts and returns success   |

## Project Structure

```
quasar-hello-world/
├── Cargo.toml        # Rust dependencies and build config
├── Quasar.toml       # Quasar project configuration
└── src/
    ├── lib.rs        # Program logic and account definitions
    └── tests.rs      # QuasarSVM integration tests
```

## Building

```sh
quasar build
```

This compiles the program as a `cdylib` targeting the Solana runtime and auto-generates typed client code in `target/client/`.

## Testing

Tests run against [QuasarSVM](https://github.com/blueshift-gg/quasar-svm), a local test harness that requires no validator.

```sh
quasar test
```

## Learn More

- [Quasar Documentation](https://quasar-lang.com/docs)
- [Quasar GitHub](https://github.com/blueshift-gg/quasar)
