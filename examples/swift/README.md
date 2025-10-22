# Swift Examples for Dojo Client

This directory contains Swift examples demonstrating how to use the Dojo Swift bindings to interact with a Torii server.

## Prerequisites

- Swift 5.9+ (with async/await support)
- macOS (for the `.dylib` library)
- A running Torii server

## Building the Bindings

Before running the examples, you need to build the Rust library and generate the Swift bindings:

```bash
# From the repository root
cd /Users/nasr/Documents/development.nosync/dojo.c

# Build the Rust library in release mode
cargo build --release

# Generate Swift bindings
cargo run --bin uniffi-bindgen-swift --release -- target/release/libdojo_c.dylib bindings/swift
```

This will generate:
- `bindings/swift/DojoEngine.swift` - The Swift interface
- `bindings/swift/DojoEngineFFI.h` - The C header file
- `bindings/swift/DojoEngineFFI.modulemap` - The module map

## Running the Examples

### Fetch Entities Example

This example connects to a Torii server and fetches entities with pagination.

**Basic usage:**
```bash
./run_fetch_entities.sh
```

**With custom Torii URL:**
```bash
./run_fetch_entities.sh http://localhost:8080
```

**With custom Torii URL and world address:**
```bash
./run_fetch_entities.sh http://localhost:8080 0x1234567890abcdef
```

### Manual Execution

If you prefer to run the Swift script manually, you need to specify the library paths:

```bash
swift \
  -I bindings/swift \
  -L target/release \
  -ldojo_c \
  examples/swift/fetch_entities.swift \
  http://localhost:8080 \
  0x0
```

**Note:** You may need to import the generated Swift module. The easiest way is to use the provided shell script `run_fetch_entities.sh`.

## Examples Overview

### 1. fetch_entities.swift

Demonstrates basic entity querying:
- Connect to a Torii server
- Create a query with pagination
- Fetch entities
- Display entity and model information

Key features:
- Async/await syntax
- Error handling
- Pagination support
- Model traversal

## Swift API Overview

### ToriiClient

The main client interface:

```swift
// Create a client with default config (4MB max message size)
let client = try await ToriiClient(toriiUrl: "http://localhost:8080")

// Create a client with custom max message size
let client = try await ToriiClient.newWithConfig(
    toriiUrl: "http://localhost:8080",
    maxMessageSize: 8 * 1024 * 1024  // 8MB
)
```

### Query Types

```swift
// Create a query for entities
let query = Query(
    worldAddresses: [],           // Empty = all worlds
    pagination: Pagination(
        cursor: nil,              // Start from beginning
        limit: 10,                // Number of items
        direction: .forward,      // Direction
        orderBy: [
            OrderBy(
                field: "created_at",
                direction: .desc
            )
        ]
    ),
    clause: nil,                  // Filtering clause
    noHashedKeys: false,
    models: [],                   // Empty = all models
    historical: false
)

// Fetch entities
let page = try await client.entities(query: query)
```

### Pagination

```swift
struct Pagination {
    let cursor: String?
    let limit: UInt32
    let direction: PaginationDirection
    let orderBy: [OrderBy]
}

enum PaginationDirection {
    case forward
    case backward
}

struct OrderBy {
    let field: String
    let direction: OrderDirection
}

enum OrderDirection {
    case asc
    case desc
}
```

### Entity Structure

```swift
struct Entity {
    let worldAddress: String
    let hashedKeys: String
    let createdAt: String
    let updatedAt: String
    let executedAt: String
    let models: [Model]
}

struct Model {
    let name: String
    let children: [Member]
    let layout: Vec<u8>?
    let useLegacyStore: Bool
}
```

## Error Handling

All async methods can throw `DojoError`:

```swift
do {
    let page = try await client.entities(query: query)
    // Process results
} catch DojoError.ConnectionError {
    print("Failed to connect to Torii server")
} catch DojoError.QueryError {
    print("Query failed")
} catch {
    print("Unexpected error: \(error)")
}
```

## Troubleshooting

### Library Not Found

If you get an error about the library not being found:

```
error: unable to load dynamic library 'libdojo_c.dylib'
```

**Solution:** Make sure you've built the library and you're running from the correct directory:
```bash
cargo build --release
./examples/swift/run_fetch_entities.sh
```

### Module Not Found

If Swift can't find the DojoEngine module:

```
error: no such module 'DojoEngine'
```

**Solution:** Generate the Swift bindings:
```bash
cargo run --bin uniffi-bindgen-swift --release -- target/release/libdojo_c.dylib bindings/swift
```

### Connection Errors

If you get connection errors:

**Solution:** Ensure your Torii server is running:
```bash
# Check if Torii is running
curl http://localhost:8080

# Or start your Torii server
torii --world 0x... --rpc https://...
```

## Next Steps

After trying these examples, you can explore:

1. **Subscriptions**: Real-time updates for entities, tokens, and events
2. **Custom Queries**: Advanced filtering with clauses
3. **Token Operations**: Query tokens, balances, and transfers
4. **Transactions**: Query and monitor transactions
5. **SQL Queries**: Direct SQL queries against the Torii database

## Additional Resources

- [Dojo Documentation](https://book.dojoengine.org/)
- [Torii Documentation](https://book.dojoengine.org/toolchain/torii/)
- [Swift Async/Await Guide](https://docs.swift.org/swift-book/LanguageGuide/Concurrency.html)

