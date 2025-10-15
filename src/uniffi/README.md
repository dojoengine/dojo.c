# UniFFI Types - Organized by Category

This directory contains UniFFI-compatible Rust types for Dojo, organized by functional category for better maintainability and discoverability.

## File Structure

```
src/uniffi/
├── mod.rs              # Module definitions and re-exports
├── core.rs             # Core types and utilities
├── achievement.rs      # Achievement system types
├── activity.rs         # Activity tracking types
├── aggregation.rs      # Data aggregation types
├── contract.rs         # Smart contract types
├── controller.rs       # Controller/account types
├── entity.rs           # Entity, Model, and World types
├── event.rs            # Event and Message types
├── query.rs            # Query and filtering types
├── schema.rs           # Schema definition types
├── token.rs            # Token and NFT types
└── transaction.rs      # Transaction types
```

## Module Organization

### `core.rs` - Core Types and Utilities
The foundation module containing:
- **Type Definitions**: `FieldElement`, `U256` (as hex strings)
- **Error Handling**: `DojoError` enum
- **Pagination**: `Pagination`, `PaginationDirection`, `OrderBy`, `OrderDirection`
- **Common Types**: `Signature`, `Call`, `BlockId`, `BlockTag`
- **Helper Functions**: Conversion between internal types and strings

**Key Types:**
- `FieldElement` - String representation of Starknet field elements
- `U256` - String representation of 256-bit unsigned integers
- `DojoError` - Comprehensive error handling
- `Pagination` - Cursor-based pagination support

### `controller.rs` - Controller Types
Account and controller management:
- `Controller` - Account controller information
- `ControllerQuery` - Query for controllers

**Use Cases:**
- Account management
- Username lookups
- Deployment tracking

### `token.rs` - Token Types
ERC20, ERC721, and ERC1155 token support:
- `Token` - Token information
- `TokenBalance` - Token balance per account
- `TokenContract` - Token contract metadata
- `TokenTransfer` - Transfer event data
- `TokenQuery`, `TokenBalanceQuery`, `TokenContractQuery`, `TokenTransferQuery`
- `AttributeFilter` - NFT attribute filtering

**Use Cases:**
- Token balance queries
- NFT metadata retrieval
- Transfer history
- Token searches with filters

### `contract.rs` - Contract Types
Smart contract information and queries:
- `Contract` - Contract metadata
- `ContractType` - Enum for contract types (WORLD, ERC20, ERC721, etc.)
- `ContractQuery` - Query contracts by address or type

**Use Cases:**
- Contract discovery
- Type-based filtering
- Deployment information

### `transaction.rs` - Transaction Types
Transaction data and queries:
- `Transaction` - Full transaction information
- `TransactionCall` - Individual contract call
- `TransactionFilter` - Advanced transaction filtering
- `TransactionQuery` - Query transactions
- `CallType` - Execute vs ExecuteFromOutside

**Use Cases:**
- Transaction history
- Call analysis
- Block explorer functionality

### `schema.rs` - Schema Types
Type system and schema definitions:
- `Primitive` - Primitive type values (integers, felts, bools, etc.)
- `Ty` - Type definitions (Struct, Enum, Array, Tuple, etc.)
- `Struct` - Struct definition
- `Enum` - Enum definition
- `Member` - Struct/enum member
- `MemberValue` - Runtime value
- `ValueType` - Value type variants

**Use Cases:**
- Schema introspection
- Dynamic typing
- Model definitions
- Data serialization

### `query.rs` - Query Types
Powerful querying and filtering:
- `Query` - Main query structure
- `Clause` - Query clauses (HashedKeys, Keys, Member, Composite)
- `KeysClause` - Key-based filtering
- `MemberClause` - Member-based filtering
- `CompositeClause` - Logical composition (AND/OR)
- `PatternMatching` - Key pattern matching
- `ComparisonOperator` - Rich comparison operators
- `LogicalOperator` - AND/OR operators

**Use Cases:**
- Entity queries
- Complex filtering
- Model data retrieval
- Historical queries

### `entity.rs` - Entity Types
Core game entity types:
- `Entity` - Game entity with models
- `Model` - Model definition
- `World` - World state

**Use Cases:**
- Entity management
- Model introspection
- World state queries

### `event.rs` - Event Types
Blockchain event handling:
- `Event` - Starknet event
- `Message` - Signed message

**Use Cases:**
- Event listening
- Message verification
- Event history

### `aggregation.rs` - Aggregation Types
Data aggregation and leaderboards:
- `AggregationEntry` - Aggregated data entry
- `AggregationQuery` - Query aggregations

**Use Cases:**
- Leaderboards
- Statistics
- Rankings

### `activity.rs` - Activity Types
Player activity tracking:
- `Activity` - Player activity session
- `ActivityQuery` - Query player activity
- `ActionCount` - Action frequency

**Use Cases:**
- Player analytics
- Session tracking
- Engagement metrics

### `achievement.rs` - Achievement Types
Achievement and progression system:
- `Achievement` - Achievement definition
- `AchievementTask` - Individual task
- `PlayerAchievementProgress` - Player's progress
- `PlayerAchievementStats` - Player's achievement stats
- `AchievementProgression` - Detailed progression
- `TaskProgress` - Task completion status
- Corresponding query types

**Use Cases:**
- Achievement systems
- Progression tracking
- Player rewards
- Gamification

## Usage Examples

### Querying Entities
```rust
use uniffi::{Query, Clause, MemberClause, ComparisonOperator, Primitive};

let query = Query {
    world_addresses: vec!["0x123...".to_string()],
    clause: Some(Clause::Member(MemberClause {
        model: "Player".to_string(),
        member: "level".to_string(),
        operator: ComparisonOperator::Gte,
        value: MemberValue::Primitive(Primitive::U32(10)),
    })),
    pagination: Default::default(),
    no_hashed_keys: false,
    models: vec!["Player".to_string()],
    historical: false,
};
```

### Token Balance Query
```rust
use uniffi::{TokenBalanceQuery, Pagination};

let query = TokenBalanceQuery {
    contract_addresses: vec!["0xabc...".to_string()],
    account_addresses: vec!["0xdef...".to_string()],
    token_ids: vec![],
    pagination: Pagination::default(),
};
```

### Achievement Progress
```rust
use uniffi::{PlayerAchievementQuery, Pagination};

let query = PlayerAchievementQuery {
    world_addresses: vec!["0x123...".to_string()],
    namespaces: vec!["game".to_string()],
    player_addresses: vec!["0xplayer...".to_string()],
    pagination: Pagination::default(),
};
```

## Design Principles

### 1. **Category-Based Organization**
Types are grouped by their domain purpose rather than technical structure:
- Easy to find related types
- Clear separation of concerns
- Logical grouping for documentation

### 2. **Minimal Dependencies**
Each module imports only what it needs:
- `core` has no internal dependencies
- Other modules depend on `core`
- Cross-references are minimal and explicit

### 3. **Consistent Patterns**
All modules follow the same patterns:
- Types use `FieldElement` and `U256` as strings
- Conversions to/from proto types
- Clear documentation of purpose

### 4. **Self-Documenting**
File names and organization make the purpose clear:
- `token.rs` - obviously token-related
- `achievement.rs` - clearly achievement system
- No need to read code to understand scope

## Adding New Types

When adding new types:

1. **Determine Category**: Which module does it belong to?
2. **Create/Update Module**: Add to existing or create new category file
3. **Update mod.rs**: Add module declaration and re-exports
4. **Update UDL**: Add type definitions to `src/dojo.udl`
5. **Document**: Update this README

Example - Adding a new "Quest" system:

```rust
// src/uniffi/quest.rs
use super::core::*;

#[derive(Debug, Clone)]
pub struct Quest {
    pub id: String,
    pub name: String,
    // ...
}
```

Update `mod.rs`:
```rust
pub mod quest;
pub use quest::*;
```

## Benefits of This Organization

### For Developers
- ✅ Quick discovery of related types
- ✅ Easy navigation
- ✅ Clear module boundaries
- ✅ Reduced cognitive load

### For Maintenance
- ✅ Easy to add new categories
- ✅ Changes are localized
- ✅ Dependencies are explicit
- ✅ Better IDE support

### For Documentation
- ✅ Self-organizing
- ✅ Category-based docs
- ✅ Clear examples per category
- ✅ Easy to generate docs

## See Also

- [UniFFI Documentation](https://mozilla.github.io/uniffi-rs/)
- [UNIFFI_CONVERSION.md](../../UNIFFI_CONVERSION.md) - Detailed conversion guide
- [CONVERSION_SUMMARY.md](../../CONVERSION_SUMMARY.md) - Complete type mapping
- [dojo.udl](../dojo.udl) - UniFFI interface definition

