# dojo.c

This package provides C, C++, and JavaScript bindings for the Torii Client SDK and starknet-rs library using [Diplomat](https://github.com/rust-diplomat/diplomat).

**⚠️ Migration Notice**: This project has been migrated from `cbindgen`/`wasm-bindgen` to `diplomat` for unified FFI generation. See [DIPLOMAT_MIGRATION.md](DIPLOMAT_MIGRATION.md) for details.

## Features

- **Unified FFI**: Single Rust API generates bindings for C, C++, and JavaScript
- **Type-Safe**: Automatic memory management and type checking
- **Modern**: Clean API with proper error handling
- **Well-Documented**: Auto-generated documentation for all languages

## Building

### Prerequisites
```bash
cargo install diplomat-tool
```

### Build Library and Generate Bindings
```bash
# Build Rust library and generate C/C++/JS bindings
cargo build --release

# Bindings are automatically generated in:
# - bindings/c/    (C headers)
# - bindings/cpp/  (C++ headers)
# - bindings/js/   (JavaScript/TypeScript modules)
```

### Platform-Specific Builds

#### Linux
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

#### macOS (Apple Silicon)
```bash
cargo build --release
```

#### WebAssembly
```bash
cargo build --release --target wasm32-unknown-unknown
# JS bindings support both native and WASM targets
```

## Usage Examples

### C
```c
#include "bindings/c/ToriiClient.h"
#include "bindings/c/SigningKey.h"

// Create a Torii client
ToriiClient_new_result result = ToriiClient_new("http://localhost:8080");
if (result.is_ok) {
    ToriiClient* client = result.ok;
    // Use client...
    ToriiClient_destroy(client);
}
```

See [examples/diplomat_example.c](examples/diplomat_example.c) for a complete example.

### C++
```cpp
#include "bindings/cpp/ToriiClient.hpp"
#include "bindings/cpp/SigningKey.hpp"

auto client = ToriiClient::new_("http://localhost:8080");
if (client.is_ok()) {
    // Use client...
}
```

### JavaScript/TypeScript
```javascript
import { ToriiClient, SigningKey } from './bindings/js/index.mjs';

const client = ToriiClient.new_("http://localhost:8080");
const info = client.info();
console.log(info);
```

See [examples/diplomat_example.js](examples/diplomat_example.js) for a complete example.

## Building Examples

### C Example
```bash
cargo build --release
clang examples/diplomat_example.c \
  -I bindings/c \
  -L target/release \
  -ldojo_c \
  -o diplomat_example

# Run (macOS)
DYLD_LIBRARY_PATH=target/release ./diplomat_example

# Run (Linux)
LD_LIBRARY_PATH=target/release ./diplomat_example
```

### JavaScript Example
```bash
node examples/diplomat_example.js
```

## API Documentation

Generated bindings include inline documentation. Key modules:

- **ToriiClient**: Torii server client for querying entities
- **SigningKey/VerifyingKey**: Cryptographic key management
- **FieldElement**: Starknet field element operations
- **Account**: Starknet account management
- **Provider**: JSON-RPC provider for blockchain interaction

## Migration Guide

If you're migrating from the old `cbindgen`/`wasm-bindgen` API, see [DIPLOMAT_MIGRATION.md](DIPLOMAT_MIGRATION.md) for:
- Architecture changes
- API differences
- Migration patterns
- Breaking changes

## Project Status

✅ **Completed**:
- Core types (FieldElement, U256, Call, Signature)
- Cryptographic operations
- Basic Torii client
- Account management
- Provider integration
- C/C++/JavaScript binding generation

🚧 **In Progress**:
- Entity query APIs
- Event subscriptions
- Token operations
- Comprehensive examples
- Full test coverage

## Contributing

When adding new FFI functions:

1. Add to appropriate module in `src/ffi/`
2. Use `#[diplomat::bridge]` and `#[diplomat::opaque]` annotations
3. Run `cargo build` to regenerate bindings
4. Test in target language (C/C++/JS)

See [DIPLOMAT_MIGRATION.md](DIPLOMAT_MIGRATION.md) for patterns and best practices.
