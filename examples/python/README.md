# Dojo Python Examples

Examples demonstrating how to use the Dojo Python bindings to interact with Torii servers.

## Prerequisites

1. Build the Dojo library:
   ```bash
   cd ../..
   cargo build --release
   ```

2. Generate Python bindings:
   ```bash
   cargo run --bin uniffi-bindgen-python --release -- \
       target/release/libdojo_c.dylib bindings/python
   ```

3. Make sure you have a Torii server running (default: `http://localhost:8080`)

## Examples

### 1. Fetch Entities (`fetch_entities.py`)

Queries and displays entities from a Torii server.

**Usage:**
```bash
python3 fetch_entities.py [torii_url] [world_address]
```

**Example:**
```bash
python3 fetch_entities.py http://localhost:8080 0x1234...
```

**What it does:**
- Connects to a Torii server
- Fetches the first 10 entities
- Displays entity metadata and model information
- Shows pagination cursor for fetching more

### 2. Subscribe to Entities (`subscribe_entities.py`)

Subscribes to real-time entity updates using callbacks.

**Usage:**
```bash
python3 subscribe_entities.py [torii_url] [world_address]
```

**Example:**
```bash
python3 subscribe_entities.py http://localhost:8080 0x1234...
```

**What it does:**
- Connects to a Torii server
- Subscribes to entity updates
- Prints updates in real-time as they occur
- Press Ctrl+C to unsubscribe and exit

## Available Client Methods

The `ToriiClient` provides many methods for interacting with Dojo:

### Queries
- `entities(query)` - Query entities
- `event_messages(query)` - Query event messages
- `worlds(world_addresses)` - Get world metadata
- `controllers(query)` - Query controllers
- `contracts(query)` - Query contracts
- `tokens(query)` - Query tokens
- `token_balances(query)` - Query token balances
- `token_contracts(query)` - Query token contracts
- `token_transfers(query)` - Query token transfers
- `transactions(query)` - Query transactions
- `aggregations(query)` - Query aggregations (leaderboards, stats)
- `activities(query)` - Query activities (user sessions)
- `achievements(query)` - Query achievements
- `player_achievements(query)` - Query player achievement progress
- `starknet_events(query)` - Query raw Starknet events
- `sql(query)` - Execute SQL query

### Publishing
- `publish_message(message)` - Publish an offchain message
- `publish_message_batch(messages)` - Publish multiple messages

### Subscriptions
- `subscribe_entity_updates(clause, world_addresses, callback)` - Subscribe to entity updates
- `subscribe_token_balance_updates(contract_addresses, account_addresses, token_ids, callback)` - Subscribe to token balance changes
- `subscribe_token_updates(contract_addresses, token_ids, callback)` - Subscribe to token metadata updates
- `subscribe_transaction_updates(filter, callback)` - Subscribe to transaction updates
- `subscribe_event_updates(keys, callback)` - Subscribe to Starknet event updates
- `cancel_subscription(subscription_id)` - Cancel a subscription

## Type Aliases

For convenience:
- `FieldElement` = `str` (hex string like "0x1234...")
- `U256` = `str` (hex string like "0xabcd...")

All methods are async and should be called with `await`.

## Error Handling

All errors are raised as `DojoError` exceptions with specific types:
- `DojoError.ClientError`
- `DojoError.ConnectionError`
- `DojoError.QueryError`
- `DojoError.SubscriptionError`
- etc.

## More Examples

Check the main README for Swift and Kotlin examples.

