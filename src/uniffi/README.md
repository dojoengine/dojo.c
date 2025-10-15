# UniFFI Bindings

This module provides foreign function interface (FFI) bindings for Dojo using Mozilla's [UniFFI](https://mozilla.github.io/uniffi-rs/) framework.

## Structure

```
src/
├── dojo.udl            # UniFFI interface definition (required at crate root)
├── uniffi/             # UniFFI implementation
│   ├── mod.rs          # Main module definition
│   ├── client.rs       # ToriiClient implementation with subscription support
│   ├── types/          # Type definitions organized by domain
│   │   ├── mod.rs      # Types module definition
│   │   ├── core.rs     # Core types (FieldElement, U256, DojoError, Pagination)
│   │   ├── achievement.rs  # Achievement and player achievement types
│   │   ├── activity.rs     # Activity tracking types
│   │   ├── aggregation.rs  # Aggregation (leaderboards, stats) types
│   │   ├── contract.rs     # Contract query types
│   │   ├── controller.rs   # Controller types
│   │   ├── entity.rs       # Entity, Model, and World types
│   │   ├── event.rs        # Event and event query types
│   │   ├── query.rs        # Query types (Clause, KeysClause, etc.)
│   │   ├── schema.rs       # Schema types (Ty, Struct, Enum, etc.)
│   │   ├── token.rs        # Token and token-related types
│   │   └── transaction.rs  # Transaction types and filters
│   └── README.md       # This file
└── bin/                # Binding generator binaries
    ├── uniffi-bindgen-swift.rs
    ├── uniffi-bindgen-kotlin.rs
    └── uniffi-bindgen-python.rs
```

**Note:** The `dojo.udl` file must be in `src/` (not `src/uniffi/`) because UniFFI requires it to be at the crate root level.

## Supported Languages

- **Swift** - iOS/macOS applications
- **Kotlin** - Android applications  
- **Python** - Python applications and scripts

## Generating Bindings

Use the provided bindgen binaries:

```bash
# Swift
cargo run --bin uniffi-bindgen-swift --release -- target/release/libdojo_c.dylib bindings/swift --swift-sources

# Kotlin  
cargo run --bin uniffi-bindgen-kotlin --release -- target/release/libdojo_c.dylib bindings/kotlin

# Python
cargo run --bin uniffi-bindgen-python --release -- target/release/libdojo_c.dylib bindings/python
```

## Features

### ToriiClient

The main client interface provides:

- **Queries**: entities, events, tokens, transactions, controllers, contracts, etc.
- **Subscriptions**: Real-time updates via callbacks
- **Message Publishing**: Submit offchain messages to the world
- **SQL Queries**: Direct database queries

### Subscriptions

Subscriptions use callbacks for real-time updates:

- `subscribe_entity_updates` - Entity state changes
- `subscribe_token_balance_updates` - Token balance changes
- `subscribe_token_updates` - Token metadata updates
- `subscribe_transaction_updates` - Transaction updates
- `subscribe_event_updates` - Starknet event updates
- `cancel_subscription` - Cancel an active subscription

### Type System

All types are automatically converted between Rust and target languages:

- **FieldElement**: Starknet field element (represented as hex string)
- **U256**: 256-bit unsigned integer (represented as hex string)
- **Enums**: Rust enums → Swift enums / Kotlin sealed classes / Python classes
- **Structs**: Rust structs → Swift structs / Kotlin data classes / Python dataclasses
- **Options**: `Option<T>` → nullable types in target languages
- **Errors**: `DojoError` enum for all error cases

## UDL Definition

The interface is defined in `src/dojo.udl` using UniFFI Definition Language.

