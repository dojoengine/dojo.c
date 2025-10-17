# Swift Bridge for Torii Client

This document describes the Swift bindings for the Torii client implemented using `swift-bridge`.

## Overview

The Swift bridge provides async/await support for all Torii client methods, enabling seamless integration with Swift applications. The implementation uses transparent structs and proper type conversions between Rust and Swift.

## Installation

Add the `async` feature to `swift-bridge` in `Cargo.toml`:

```toml
swift-bridge = { version = "0.1.57", features = ["async"] }
```

## Generated Files

After building, the following files are generated in `generated/`:
- `dojo-c/dojo-c.swift` - Swift API
- `dojo-c/dojo-c.h` - C header for bridging
- `SwiftBridgeCore.swift` - Core bridge utilities
- `SwiftBridgeCore.h` - Core bridge headers

## Core Types

### ToriiClient

The main client type for interacting with Torii.

**Initialization:**
```swift
// Default configuration
let client = try await newToriiClient(toriiUrl: "http://localhost:8080")

// Custom configuration
let client = try await newToriiClientWithConfig(
    toriiUrl: "http://localhost:8080",
    maxMessageSize: 4194304
)
```

### FeltBridge

Represents a Starknet Felt value as a hex string.

```swift
struct FeltBridge {
    var hex: String  // Format: "0x..."
}
```

### PageBridge

Wrapper for paginated results containing JSON-serialized data.

```swift
struct PageBridge {
    var data: String  // JSON string
}
```

### QueryBridge

Basic query parameters for entity queries.

```swift
struct QueryBridge {
    var limit: UInt32
    var cursor: String  // Empty string for first page, use returned cursor for next page
    var worldAddresses: RustVec<FeltBridge>
    var dontIncludeHashedKeys: Bool
}
```

### WorldBridge

Represents world metadata.

```swift
struct WorldBridge {
    var worldAddress: FeltBridge
    var modelsJson: String  // JSON-serialized models
}
```

### Subscription

Represents an active subscription to real-time updates.

```swift
// Get subscription ID
let id = subscription.id()

// Cancel subscription (consumes the subscription)
subscription.cancel()
```

### ToriiClientError

Error type with descriptive message.

```swift
do {
    try await client.entities(query: query)
} catch let error as ToriiClientError {
    print("Error: \(error.message())")
}
```

## API Methods

### World Operations

#### worlds
```swift
func worlds(
    worldAddresses: RustVec<FeltBridge>
) async throws -> RustVec<WorldBridge>
```

Get metadata for specified worlds.

### Message Operations

#### publishMessage
```swift
func publishMessage(
    messageJson: String
) async throws -> String
```

Publish a single message. `messageJson` should be JSON representation of `Message`.

#### publishMessageBatch
```swift
func publishMessageBatch(
    messagesJson: String
) async throws -> String
```

Publish multiple messages. Returns JSON array of entity IDs.

### Entity Operations

#### entities
```swift
func entities(
    query: QueryBridge
) async throws -> PageBridge
```

Query entities. Returns JSON-serialized `Page<Entity>`.

#### eventMessages
```swift
func eventMessages(
    query: QueryBridge
) async throws -> PageBridge
```

Query event messages. Returns JSON-serialized `Page<Entity>`.

#### starknetEvents
```swift
func starknetEvents(
    keysJson: String,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query raw Starknet events. `keysJson` should be JSON representation of `KeysClause`.

### Token Operations

#### tokens
```swift
func tokens(
    contractAddresses: RustVec<FeltBridge>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query tokens for specified contract addresses.

#### tokenBalances
```swift
func tokenBalances(
    accountAddresses: RustVec<FeltBridge>,
    contractAddresses: RustVec<FeltBridge>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query token balances for accounts and contracts.

#### tokenContracts
```swift
func tokenContracts(
    contractAddresses: RustVec<FeltBridge>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query token contract metadata.

#### tokenTransfers
```swift
func tokenTransfers(
    accountAddresses: RustVec<FeltBridge>,
    contractAddresses: RustVec<FeltBridge>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query token transfers.

### Transaction Operations

#### transactions
```swift
func transactions(
    filterJson: String,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query transactions. `filterJson` should be JSON representation of `TransactionFilter` (optional).

### Contract & Controller Operations

#### contracts
```swift
func contracts(
    contractAddresses: RustVec<FeltBridge>
) async throws -> String
```

Query contracts. Returns JSON array of `Contract`.

#### controllers
```swift
func controllers(
    contractAddresses: RustVec<FeltBridge>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query controllers.

### Aggregation Operations

#### aggregations
```swift
func aggregations(
    aggregatorIds: RustVec<String>,
    entityIds: RustVec<String>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query aggregations (leaderboards, stats, rankings).

### Activity Operations

#### activities
```swift
func activities(
    worldAddresses: RustVec<FeltBridge>,
    namespaces: RustVec<String>,
    callerAddresses: RustVec<FeltBridge>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query user activity/session tracking.

### Achievement Operations

#### achievements
```swift
func achievements(
    worldAddresses: RustVec<FeltBridge>,
    namespaces: RustVec<String>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query achievements.

#### playerAchievements
```swift
func playerAchievements(
    worldAddresses: RustVec<FeltBridge>,
    namespaces: RustVec<String>,
    playerAddresses: RustVec<FeltBridge>,
    limit: UInt32,
    cursor: String
) async throws -> PageBridge
```

Query player-specific achievement data.

### SQL Operations

#### sql
```swift
func sql(query: String) async throws -> String
```

Execute raw SQL query. Returns JSON-serialized array of `SqlRow`.

### Subscription Operations

#### onEntityUpdated
```swift
func onEntityUpdated(
    clauseJson: String,
    worldAddresses: RustVec<FeltBridge>
) async throws -> Subscription
```

Subscribe to entity updates. `clauseJson` should be JSON representation of `Clause` (optional).

#### onStarknetEvent
```swift
func onStarknetEvent(
    keysJson: String
) async throws -> Subscription
```

Subscribe to Starknet events. `keysJson` should be JSON array of `KeysClause`.

## Example Usage

```swift
import Foundation

// Initialize client
let client = try await newToriiClient(toriiUrl: "http://localhost:8080")

// Create Felt from hex
let worldAddress = FeltBridge(hex: "0x1234...")

// Query entities
let query = QueryBridge(
    limit: 10,
    cursor: "",  // Empty for first page
    worldAddresses: RustVec([worldAddress]),
    dontIncludeHashedKeys: false
)

let page = try await client.entities(query: query)

// Parse JSON response
if let data = page.data.data(using: .utf8),
   let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any] {
    print("Entities: \(json)")
    
    // Get next cursor for pagination
    if let nextCursor = json["next_cursor"] as? String, !nextCursor.isEmpty {
        // Fetch next page
        let nextQuery = QueryBridge(
            limit: 10,
            cursor: nextCursor,
            worldAddresses: RustVec([worldAddress]),
            dontIncludeHashedKeys: false
        )
        let nextPage = try await client.entities(query: nextQuery)
    }
}

// Query tokens
let contractAddr = FeltBridge(hex: "0xabcd...")
let tokens = try await client.tokens(
    contractAddresses: RustVec([contractAddr]),
    limit: 20,
    cursor: ""  // Empty for first page
)

// Subscribe to updates
let subscription = try await client.onEntityUpdated(
    clauseJson: "",  // Empty for all entities
    worldAddresses: RustVec([worldAddress])
)

print("Subscription ID: \(subscription.id())")

// Cancel when done
subscription.cancel()
```

## Type Conversions

### Felt Values

Felt values are represented as hex strings:
```swift
let felt = FeltBridge(hex: "0x1234567890abcdef")
```

### Vec to RustVec

Swift arrays need to be wrapped in `RustVec`:
```swift
let addresses = RustVec([
    FeltBridge(hex: "0x1"),
    FeltBridge(hex: "0x2")
])
```

### JSON Serialization

Complex query parameters and results use JSON:
```swift
// Create JSON query
let clauseDict: [String: Any] = [
    "Keys": [
        "keys": ["0x123"],
        "pattern_matching": "FixedLen",
        "models": []
    ]
]
let clauseJson = try JSONSerialization.data(with: clauseDict).utf8String

// Parse JSON result
let pageData = try JSONDecoder().decode(YourType.self, from: page.data.data(using: .utf8)!)
```

## Notes

- All methods are `async` and can throw `ToriiClientError`
- Use `try await` when calling async methods
- `Subscription.cancel()` consumes the subscription (owned `self`)
- Complex types (Clause, Filter, etc.) are passed as JSON strings
- Results are typically returned as JSON strings in `PageBridge.data`
- Pagination uses `limit` and `offset` parameters
- Empty string for JSON parameters means "no filter" or "default"

## Building

```bash
cargo build --lib
```

Generated Swift files will be in `generated/dojo-c/`.

## Integration

1. Add generated Swift files to your Xcode project
2. Link against the compiled Rust library (`libdojo_c.a` or `.dylib`)
3. Import the module: `import DojoC`
4. Use async/await with the client

## Limitations

- Stream handling for subscriptions is simplified (TODO: proper stream handling)
- Some nested struct types use JSON serialization for simplicity
- swift_bridge doesn't support all Rust types directly (see workarounds in implementation)

