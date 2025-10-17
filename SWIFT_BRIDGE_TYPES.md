# Swift Bridge Types Documentation

This document describes all the Swift bridge types available for working with the Torii client.

## Core Types

### FeltBridge
Represents a Starknet field element (Felt252).

```swift
public struct FeltBridge {
    var hex: RustString  // Hex string representation starting with "0x"
}
```

### U256Bridge
Represents a 256-bit unsigned integer.

```swift
public struct U256Bridge {
    var hex: RustString  // Hex string representation starting with "0x"
}
```

### PageBridge
Generic pagination wrapper. The `data` field contains JSON-serialized page results.

```swift
public struct PageBridge {
    var data: RustString  // JSON string containing paginated items
}
```

## Entity & Model Types

### EntityBridge
Represents a Dojo entity with its models and metadata.

```swift
public struct EntityBridge {
    var world_address: FeltBridge
    var hashed_keys: FeltBridge
    var models_json: RustString  // JSON array of model structs
    var created_at: UInt64
    var updated_at: UInt64
    var executed_at: UInt64
}
```

### ModelBridge
Represents a Dojo model definition.

```swift
public struct ModelBridge {
    var world_address: FeltBridge
    var namespace: RustString
    var name: RustString
    var selector: FeltBridge
    var packed_size: UInt32
    var unpacked_size: UInt32
    var class_hash: FeltBridge
    var contract_address: FeltBridge
    var layout_json: RustString  // JSON serialized layout
    var schema_json: RustString  // JSON serialized schema
    var use_legacy_store: Bool
}
```

### WorldBridge
Represents world metadata with its models.

```swift
public struct WorldBridge {
    var world_address: FeltBridge
    var models_json: RustString  // JSON map of models
}
```

## Token Types

### TokenBridge
Represents a token (ERC20/ERC721/ERC1155).

```swift
public struct TokenBridge {
    var contract_address: FeltBridge
    var token_id: RustString  // Optional U256 as JSON, empty if ERC20
    var name: RustString
    var symbol: RustString
    var decimals: UInt8
    var metadata: RustString  // JSON metadata
    var total_supply: RustString  // Optional U256 as JSON
}
```

### TokenBalanceBridge
Represents a token balance for an account.

```swift
public struct TokenBalanceBridge {
    var balance: U256Bridge
    var account_address: FeltBridge
    var contract_address: FeltBridge
    var token_id: RustString  // Optional U256 as JSON, empty if ERC20
}
```

### TokenContractBridge
Represents a token contract's metadata.

```swift
public struct TokenContractBridge {
    var contract_address: FeltBridge
    var name: RustString
    var symbol: RustString
    var decimals: UInt8
    var metadata: RustString
    var token_metadata: RustString
    var total_supply: RustString  // Optional U256 as JSON
}
```

### TokenTransferBridge
Represents a token transfer event.

```swift
public struct TokenTransferBridge {
    var id: RustString
    var contract_address: FeltBridge
    var from_address: FeltBridge
    var to_address: FeltBridge
    var amount: U256Bridge
    var token_id: RustString  // Optional U256 as JSON
    var executed_at: UInt64  // Unix timestamp
    var event_id: RustString
}
```

## Contract & Controller Types

### ControllerBridge
Represents a Cartridge controller.

```swift
public struct ControllerBridge {
    var address: FeltBridge
    var username: RustString
    var deployed_at_timestamp: UInt64  // Unix timestamp
}
```

### ContractBridge
Represents a smart contract with its metadata.

```swift
public struct ContractBridge {
    var contract_address: FeltBridge
    var contract_type: RustString  // "WORLD", "ERC20", "ERC721", "ERC1155", "UDC", "OTHER"
    var head: RustString  // Optional u64 as string
    var tps: RustString  // Optional u64 as string (transactions per second)
    var last_block_timestamp: RustString  // Optional u64 as string
    var last_pending_block_tx: RustString  // Optional Felt as hex string
    var updated_at: UInt64
    var created_at: UInt64
}
```

## Transaction Types

### TransactionBridge
Represents a Starknet transaction.

```swift
public struct TransactionBridge {
    var transaction_hash: FeltBridge
    var sender_address: FeltBridge
    var calldata: RustVec<FeltBridge>
    var max_fee: FeltBridge
    var signature: RustVec<FeltBridge>
    var nonce: FeltBridge
    var block_number: UInt64
    var transaction_type: RustString  // "INVOKE", "DEPLOY", etc.
    var block_timestamp: UInt64
    var calls_json: RustString  // JSON array of TransactionCall
    var unique_models: RustVec<FeltBridge>
}
```

### TransactionCallBridge
Represents an individual call within a transaction.

```swift
public struct TransactionCallBridge {
    var contract_address: FeltBridge
    var entrypoint: RustString
    var calldata: RustVec<FeltBridge>
    var call_type: RustString  // "Execute" or "ExecuteFromOutside"
    var caller_address: FeltBridge
}
```

## Event Types

### EventBridge
Represents a raw Starknet event.

```swift
public struct EventBridge {
    var keys: RustVec<FeltBridge>
    var data: RustVec<FeltBridge>
    var transaction_hash: FeltBridge
}
```

## Aggregation & Activity Types

### AggregationEntryBridge
Represents an aggregation/leaderboard entry.

```swift
public struct AggregationEntryBridge {
    var id: RustString
    var aggregator_id: RustString
    var entity_id: RustString
    var value: U256Bridge
    var display_value: RustString
    var position: UInt64
    var model_id: RustString
    var created_at: UInt64
    var updated_at: UInt64
}
```

### ActivityBridge
Represents player activity in a game session.

```swift
public struct ActivityBridge {
    var id: RustString
    var world_address: FeltBridge
    var namespace: RustString
    var caller_address: FeltBridge
    var session_start: UInt64  // Unix timestamp
    var session_end: UInt64  // Unix timestamp
    var action_count: UInt32
    var actions_json: RustString  // JSON map of action names to counts
    var updated_at: UInt64
}
```

### ActionCountBridge
Represents the count of a specific action.

```swift
public struct ActionCountBridge {
    var action_name: RustString
    var count: UInt32
}
```

## Achievement Types

### AchievementBridge
Represents a game achievement.

```swift
public struct AchievementBridge {
    var id: RustString
    var world_address: FeltBridge
    var namespace: RustString
    var entity_id: RustString
    var hidden: Bool
    var index: UInt32
    var points: UInt32
    var start: RustString  // ISO timestamp or empty
    var end: RustString  // ISO timestamp or empty
    var group: RustString
    var icon: RustString  // URL or icon identifier
    var title: RustString
    var description: RustString
    var tasks_json: RustString  // JSON array of AchievementTask
    var data: RustString  // Additional JSON metadata
    var total_completions: UInt32
    var completion_rate: Double  // 0.0 to 1.0
    var created_at: UInt64
    var updated_at: UInt64
}
```

### AchievementTaskBridge
Represents a task within an achievement.

```swift
public struct AchievementTaskBridge {
    var task_id: RustString
    var description: RustString
    var total: UInt32  // Required count to complete
    var total_completions: UInt32  // Number of players who completed
    var completion_rate: Double
    var created_at: UInt64
}
```

### PlayerAchievementEntryBridge
Represents a player's achievement data and progress.

```swift
public struct PlayerAchievementEntryBridge {
    var player_address: FeltBridge
    var stats: PlayerAchievementStatsBridge
    var achievements_json: RustString  // JSON array of PlayerAchievementProgress
}
```

### PlayerAchievementStatsBridge
Represents a player's overall achievement statistics.

```swift
public struct PlayerAchievementStatsBridge {
    var total_points: UInt32
    var completed_achievements: UInt32
    var total_achievements: UInt32
    var completion_percentage: Double  // 0.0 to 1.0
    var last_achievement_at: RustString  // Unix timestamp as string or empty
    var created_at: UInt64
    var updated_at: UInt64
}
```

### PlayerAchievementProgressBridge
Represents a player's progress on a specific achievement.

```swift
public struct PlayerAchievementProgressBridge {
    var achievement: AchievementBridge
    var task_progress_json: RustString  // JSON array of TaskProgress
    var completed: Bool
    var progress_percentage: Double  // 0.0 to 1.0
}
```

### TaskProgressBridge
Represents a player's progress on a specific task.

```swift
public struct TaskProgressBridge {
    var task_id: RustString
    var count: UInt32  // Current progress count
    var completed: Bool
}
```

## Usage Examples

### Working with Tokens

```swift
let client = try await newToriiClient(toriiUrl: "http://localhost:8080")

// Query tokens
let contractAddr = FeltBridge(hex: "0x1234...")
let tokenPage = try await client.tokens(
    contractAddresses: RustVec([contractAddr]),
    limit: 10,
    cursor: ""
)

// Parse token data
if let data = tokenPage.data.data(using: .utf8),
   let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
   let items = json["items"] as? [[String: Any]] {
    
    for item in items {
        print("Token: \(item["name"] ?? "Unknown")")
        print("Symbol: \(item["symbol"] ?? "Unknown")")
    }
}
```

### Working with Entities

```swift
let query = QueryBridge(
    limit: 20,
    cursor: "",
    worldAddresses: RustVec([worldAddr]),
    dontIncludeHashedKeys: false
)

let entitiesPage = try await client.entities(query: query)

// Parse entities - JSON contains EntityBridge objects
if let data = entitiesPage.data.data(using: .utf8),
   let page = try? JSONDecoder().decode(Page<EntityData>.self, from: data) {
    
    for entity in page.items {
        print("Entity at: \(entity.world_address)")
        // Parse models_json to access entity models
    }
}
```

### Working with Achievements

```swift
// Query achievements
let achievementsPage = try await client.achievements(
    worldAddresses: RustVec([worldAddr]),
    namespaces: RustVec(["game"]),
    limit: 10,
    cursor: ""
)

// Parse achievements
// JSON contains array of AchievementBridge objects
```

### Working with Transactions

```swift
// Create transaction filter
let filter = [
    "caller_addresses": ["0x1234..."],
    "entrypoints": ["spawn", "move"]
]
let filterJson = String(data: try! JSONSerialization.data(withJSONObject: filter), 
                       encoding: .utf8)!

let txPage = try await client.transactions(
    filterJson: filterJson,
    limit: 20,
    cursor: ""
)

// Parse transactions - JSON contains TransactionBridge objects
```

## JSON Field Parsing

Many bridge types contain `_json` suffix fields that hold JSON-serialized complex data. To parse these:

```swift
// Example: Parsing models_json from EntityBridge
if let modelsData = entity.models_json.data(using: .utf8),
   let models = try? JSONDecoder().decode([ModelData].self, from: modelsData) {
    // Work with models array
}

// Example: Parsing actions from ActivityBridge  
if let actionsData = activity.actions_json.data(using: .utf8),
   let actions = try? JSONDecoder().decode([String: UInt32].self, from: actionsData) {
    // actions is a dictionary of action names to counts
}
```

## Type Conversions

### Felt to/from Hex String

```swift
// Create a Felt from hex
let felt = FeltBridge(hex: "0x1234567890abcdef")

// Use in API calls
let result = try await client.someMethod(address: felt)

// The hex property is a RustString, convert to Swift String
let hexString = String(felt.hex)
```

### U256 Handling

```swift
// U256 is represented as hex string
let amount = U256Bridge(hex: "0x1000000000000000000")

// Parse to BigInt or Decimal if needed
let hexValue = String(amount.hex)
// Use a library like BigInt for arithmetic
```

## Notes

- **RustString**: Swift-bridge's string type. Convert to Swift String using `String(rustString)`.
- **RustVec**: Swift-bridge's vector type. Create with `RustVec([item1, item2, ...])`.
- **Optional Fields**: Fields that are optional in Rust are represented as empty strings or "null" JSON strings.
- **Timestamps**: Unix timestamps are in seconds since epoch.
- **JSON Fields**: Fields ending with `_json` contain serialized JSON that needs to be parsed separately.

