# Dojo Swift Bridge

This document describes how to use the Dojo Swift bindings generated via `swift-bridge`.

## Overview

The Dojo Swift bridge provides native Swift interfaces for:
- **Starknet Provider**: Interact with Starknet nodes
- **Account Management**: Create and manage Starknet accounts
- **Torii Client**: Query and interact with Dojo game state
- **Cryptographic Functions**: Sign transactions, verify signatures, and compute hashes
- **Utility Functions**: Convert between Cairo and native types

## Building

### Prerequisites

- Rust toolchain (stable)
- Xcode (for Swift development)
- Cargo

### Generate Swift Bindings

When you build the project, the Swift bridge will automatically generate the Swift and C FFI code:

```bash
cargo build
```

The generated files will be placed in the `./generated` directory:
- `dojo-c.swift` - Swift interface definitions
- `dojo-c.h` - C header file for FFI

## Integration with Xcode

1. **Add Generated Files**: Add the generated Swift and C files to your Xcode project.

2. **Link Static Library**: Link against the generated static library:
   - For debug builds: `target/debug/libdojo_c.a`
   - For release builds: `target/release/libdojo_c.a`

3. **Bridging Header**: If using a bridging header, import the C header:
   ```c
   #include "dojo-c.h"
   ```

## Usage Examples

### Provider Operations

```swift
// Create a provider
let provider = try RustProvider(rpcUrl: "https://api.cartridge.gg/x/starknet/mainnet")

// Get chain ID
let chainId = try provider.chainId()
print("Chain ID: \(chainId)")

// Call a contract
let result = try provider.call(
    contractAddress: "0x1234...",
    selector: "get_balance",
    calldata: ["0x5678..."]
)
print("Call result: \(result)")

// Wait for a transaction
let success = try provider.waitForTransaction(txnHash: "0xabcd...")
print("Transaction confirmed: \(success)")
```

### Account Management

```swift
// Create an account
let account = try RustAccount(
    provider: provider,
    privateKey: "0xabcd...",
    address: "0x5678..."
)

// Get account info
let address = try account.address()
let chainId = try account.chainId()
let nonce = try account.nonce()

print("Account: \(address)")
print("Chain ID: \(chainId)")
print("Nonce: \(nonce)")

// Execute a transaction
let calls = [
    CallData(
        to: "0x1234...",
        selector: "transfer",
        calldata: ["0x5678...", "0x64", "0x0"]
    )
]
let txHash = try account.executeRaw(calls: calls)
print("Transaction hash: \(txHash)")

// Wait for transaction
let success = try provider.waitForTransaction(txnHash: txHash)
print("Transaction confirmed: \(success)")
```

### Deploy Burner Account

```swift
// Create a signing key for the burner
let burnerKey = generateSigningKey()

// Deploy burner from master account  
let burner = try RustAccount.deployBurner(
    provider: provider,
    masterAccount: masterAccount,
    signingKey: burnerKey
)

let burnerAddress = try burner.address()
print("Burner deployed at: \(burnerAddress)")
```

### Torii Client

```swift
// Create a Torii client
let client = try RustToriiClient(
    toriiUrl: "https://api.cartridge.gg/x/starknet-sepolia/torii",
    worldAddress: "0x1234..."
)

// Query entities
let entityQuery = EntityQuery(
    worldAddresses: ["0x1234..."],
    limit: 10,
    offset: 0
)
let entities = try client.getEntities(query: entityQuery)
print("Found \(entities.totalCount) entities")

// Query tokens
let tokenQuery = TokenQuery(
    contractAddresses: ["0x5678..."],
    tokenIds: [],
    limit: 10,
    offset: 0
)
let tokens = try client.getTokens(query: tokenQuery)
for token in tokens.tokens {
    print("Token: \(token.name) (\(token.symbol))")
}

// Query token balances
let balanceQuery = TokenBalanceQuery(
    contractAddresses: ["0x5678..."],
    accountAddresses: ["0xabcd..."],
    tokenIds: [],
    limit: 10,
    offset: 0
)
let balances = try client.getTokenBalances(query: balanceQuery)
for balance in balances.balances {
    print("Balance: \(balance.balance) for account \(balance.accountAddress)")
}

// Publish a message
let message = PublishMessage(
    message: "Hello, Dojo!",
    signature: ["0xabcd...", "0xef01..."],
    worldAddress: "0x1234..."
)
let messageId = try client.publishMessage(message: message)
print("Message published: \(messageId)")
```

### Cryptographic Operations

```swift
// Generate a new signing key
let privateKey = generateSigningKey()
print("Private key: \(privateKey)")

// Get verifying key
let verifyingKey = try getVerifyingKey(signingKey: privateKey)
print("Verifying key: \(verifyingKey)")

// Sign a message
let hash = "0x1234..."
let signature = try sign(privateKey: privateKey, hash: hash)
print("Signature: r=\(signature.r), s=\(signature.s)")

// Verify signature
let isValid = try verify(
    verifyingKey: verifyingKey,
    hash: hash,
    signature: signature
)
print("Signature valid: \(isValid)")
```

### Utility Functions

```swift
// Poseidon hash
let inputs = ["0x1", "0x2", "0x3"]
let hash = try poseidonHash(inputs: inputs)
print("Hash: \(hash)")

// Get selector from name
let selector = try getSelectorFromName(name: "transfer")
print("Selector: \(selector)")

// Get selector from tag
let tagSelector = getSelectorFromTag(tag: "dojo-Moves")
print("Tag selector: \(tagSelector)")

// Convert short string to felt
let felt = try cairoShortStringToFelt(str: "Hello")
print("Felt: \(felt)")

// Parse short string from felt
let string = try parseCairoShortString(felt: "0x48656c6c6f")
print("String: \(string)")

// Serialize bytearray
let serialized = try bytearraySerialize(str: "Hello, World!")
print("Serialized: \(serialized)")

// Deserialize bytearray
let deserialized = try bytearrayDeserialize(felts: serialized)
print("Deserialized: \(deserialized)")

// Compute contract address
let contractAddress = try getContractAddress(
    classHash: "0x1234...",
    salt: "0x5678...",
    constructorCalldata: ["0xabcd..."],
    deployerAddress: "0xef01..."
)
print("Contract address: \(contractAddress)")
```

### Typed Data Encoding

```swift
let typedDataJson = """
{
    "types": {
        "StarkNetDomain": [
            {"name": "name", "type": "felt"},
            {"name": "version", "type": "felt"},
            {"name": "chainId", "type": "felt"}
        ],
        "Message": [
            {"name": "content", "type": "felt"}
        ]
    },
    "primaryType": "Message",
    "domain": {
        "name": "MyApp",
        "version": "1",
        "chainId": "SN_MAIN"
    },
    "message": {
        "content": "Hello, Starknet!"
    }
}
"""

let hash = try typedDataEncode(
    typedData: typedDataJson,
    address: "0x1234..."
)
print("Typed data hash: \(hash)")
```

## Error Handling

All functions that can fail throw Swift errors. Use Swift's error handling:

```swift
do {
    let provider = try RustProvider(rpcUrl: rpcUrl)
    let chainId = try provider.chainId()
    print("Chain ID: \(chainId)")
} catch {
    print("Error: \(error)")
}
```

## Type Reference

### Core Types

- **RustToriiClient**: Opaque handle to Torii client
- **RustProvider**: Opaque handle to Starknet provider
- **RustAccount**: Opaque handle to Starknet account
- **RustSubscription**: Opaque handle to subscription

### Data Types

- **CallData**: Transaction call data
  - `to: String` - Contract address
  - `selector: String` - Function selector name
  - `calldata: [String]` - Array of calldata as hex strings

- **SignatureData**: Cryptographic signature
  - `r: String` - R component
  - `s: String` - S component

- **EntityQuery**: Query parameters for entities
  - `world_addresses: [String]` - World contract addresses
  - `limit: UInt32` - Maximum results
  - `offset: UInt32` - Pagination offset

- **TokenQuery**: Query parameters for tokens
  - `contract_addresses: [String]` - Token contract addresses
  - `token_ids: [String]` - Specific token IDs
  - `limit: UInt32` - Maximum results
  - `offset: UInt32` - Pagination offset

- **TokenBalanceQuery**: Query parameters for token balances
  - `contract_addresses: [String]` - Token contract addresses
  - `account_addresses: [String]` - Account addresses
  - `token_ids: [String]` - Specific token IDs
  - `limit: UInt32` - Maximum results
  - `offset: UInt32` - Pagination offset

## Performance Considerations

1. **Runtime Creation**: Some operations create a Tokio runtime internally. Consider caching instances when possible.

2. **String Conversions**: All hex strings should include the `0x` prefix. The library will handle conversions.

3. **Memory Management**: The Swift bridge handles memory management automatically through Rust's ownership system and Swift's ARC.

## Contributing

When adding new functions to the Swift bridge:

1. Add the function signature in the `extern "Rust"` block in `src/swift/mod.rs`
2. Implement the function in Rust below the bridge definition
3. Rebuild to regenerate Swift bindings
4. Test in Swift

## License

This project is licensed under the same terms as the main Dojo project.

