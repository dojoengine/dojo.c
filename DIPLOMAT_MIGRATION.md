# Diplomat Migration Guide

This document describes the migration of dojo.c from `cbindgen`/`wasm-bindgen` to `diplomat` for FFI generation.

## Overview

The codebase has been restructured to use [Diplomat](https://github.com/rust-diplomat/diplomat), a Rust FFI tool that generates bindings for C, C++, and JavaScript from a single source.

## Architecture Changes

### Before (cbindgen/wasm-bindgen)
```
src/
├── c/          # C FFI with manual raw pointers
│   ├── mod.rs  # 80+ extern "C" functions
│   └── types.rs # 2000+ lines of #[repr(C)] structs
├── wasm/       # Separate WASM bindings
│   ├── mod.rs  # wasm-bindgen functions
│   └── types.rs # TypeScript types
└── lib.rs
```

### After (diplomat)
```
src/
├── ffi/              # Unified diplomat FFI bridge
│   ├── mod.rs        # Module exports
│   ├── error.rs      # Error types
│   ├── types.rs      # Core types (FieldElement, U256, Call, etc.)
│   ├── crypto.rs     # Cryptographic operations
│   ├── client.rs     # Torii client
│   ├── account.rs    # Starknet accounts
│   ├── query.rs      # Query types
│   └── subscription.rs # Subscriptions
├── c/          # Legacy (kept for reference)
├── wasm/       # Legacy (kept for reference)
└── lib.rs      # Exports diplomat modules
```

## Key Diplomat Patterns

### 1. Module Structure
```rust
#[diplomat::bridge]
pub mod ffi {
    // All FFI types and functions go here
}
```

### 2. Opaque Types
Instead of `#[repr(C)]` structs, use opaque wrappers:

```rust
// OLD (cbindgen)
#[repr(C)]
pub struct FieldElement {
    pub data: [u8; 32],
}

// NEW (diplomat)
#[diplomat::opaque]
pub struct FieldElement(pub Felt);
```

### 3. String Handling
- **Input**: Use `DiplomatStr` instead of `*const c_char`
- **Output**: Use `DiplomatWrite` instead of returning `*const c_char`

```rust
// OLD
pub unsafe extern "C" fn felt_to_hex(felt: FieldElement) -> *const c_char {
    let hex = format!("{:#x}", felt.0);
    CString::new(hex).unwrap().into_raw()
}

// NEW
impl FieldElement {
    pub fn to_hex(&self, result: &mut DiplomatWrite) {
        write!(result, "{:#x}", self.0).unwrap();
    }
}
```

### 4. Error Handling
Use `Result<T, Box<ErrorType>>`:

```rust
#[diplomat::opaque]
pub struct DojoError {
    pub(crate) error_type: ErrorType,
    pub(crate) message: String,
}

pub enum ErrorType {
    ClientError,
    ParseError,
    // ...
}

impl FieldElement {
    pub fn new_from_hex(hex: &DiplomatStr) -> Result<Box<FieldElement>, Box<DojoError>> {
        let s = std::str::from_utf8(hex)?;
        let felt = Felt::from_hex(s).map_err(|e| {
            DojoError::new(ErrorType::ParseError, &format!("Invalid felt hex: {}", e))
        })?;
        Ok(Box::new(FieldElement(felt)))
    }
}
```

### 5. Async Operations
Use a lazy_static runtime:

```rust
lazy_static! {
    static ref RUNTIME: Arc<Runtime> =
        Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
}

impl ToriiClient {
    pub fn new(torii_url: &DiplomatStr) -> Result<Box<ToriiClient>, Box<DojoError>> {
        let url = std::str::from_utf8(torii_url)?.to_string();
        let client = RUNTIME.block_on(TClient::new(url))
            .map_err(|e| DojoError::new(ErrorType::ClientError, &e.to_string()))?;
        Ok(Box::new(ToriiClient { inner: Arc::new(client) }))
    }
}
```

### 6. Collections
- Use `&[Box<Type>]` for input arrays
- Create wrapper types for output collections

```rust
#[diplomat::opaque]
pub struct CallList {
    pub(crate) calls: Vec<starknet::core::types::Call>,
}

impl CallList {
    pub fn new() -> Box<CallList> {
        Box::new(CallList { calls: Vec::new() })
    }
    
    pub fn add_call(&mut self, call: &Call) {
        self.calls.push(call.0.clone());
    }
}
```

## Generated Bindings

Diplomat automatically generates bindings in three languages:

### C Bindings (`bindings/c/`)
```c
#include "ToriiClient.h"

ToriiClient_new_result client_result = ToriiClient_new("http://localhost:8080");
if (client_result.is_ok) {
    ToriiClient* client = client_result.ok;
    // Use client...
    ToriiClient_destroy(client);
}
```

### C++ Bindings (`bindings/cpp/`)
```cpp
#include "ToriiClient.hpp"

auto client = ToriiClient::new_("http://localhost:8080");
if (client.is_ok()) {
    auto& c = client.ok();
    // Use client...
}
```

### JavaScript Bindings (`bindings/js/`)
```typescript
import { ToriiClient } from './bindings/js/index.mjs';

const client = ToriiClient.new_("http://localhost:8080");
const info = client.info();
console.log(info);
```

## Build Configuration

### Cargo.toml
```toml
[dependencies]
diplomat = "0.12.0"
diplomat-runtime = "0.12.0"

[build-dependencies]
diplomat-tool = "0.12.0"
```

### build.rs
```rust
use diplomat_tool::{config::{Config, SharedConfig}, DocsUrlGenerator};

pub fn main() {
    // Generate C bindings
    diplomat_tool::gen(
        Path::new("src/lib.rs"),
        "c",
        Path::new("bindings/c"),
        &DocsUrlGenerator::default(),
        Config {
            shared_config: SharedConfig {
                lib_name: Some("dojo_c".to_string()),
                ..Default::default()
            },
            ..Config::default()
        },
        false,
    ).unwrap();
    
    // Similar for C++ and JS...
}
```

## Migration Checklist

- [x] Add diplomat dependencies to Cargo.toml
- [x] Create ffi module structure
- [x] Implement core types (FieldElement, U256, Call, Signature)
- [x] Implement error handling
- [x] Implement crypto operations (SigningKey, VerifyingKey)
- [x] Implement Torii client
- [x] Implement account management
- [x] Update build.rs to use diplomat-tool
- [x] Generate bindings for C, C++, and JavaScript
- [ ] Port remaining 80+ functions from old C module
- [ ] Add comprehensive entity query APIs
- [ ] Add token and event query APIs
- [ ] Add subscription management
- [ ] Update examples
- [ ] Update tests
- [ ] Update documentation

## Next Steps

### 1. Port Remaining Functions
The old `src/c/mod.rs` has 80+ functions. Continue migrating them following the patterns above:

- Entity queries (`client_get_entities`, etc.)
- Model operations
- Event subscriptions
- Token operations
- Controller/Session management

### 2. Add Entity Types
Create diplomat types for:
- `Entity`
- `Model`
- `Event`
- `Token`
- `TokenBalance`
- etc.

### 3. Testing
- Create integration tests
- Test C bindings with example C program
- Test C++ bindings
- Test JavaScript bindings in browser and Node.js

### 4. Documentation
- Generate API documentation
- Create usage examples for each language
- Document breaking changes from old API

## Benefits of Diplomat

1. **Single Source of Truth**: One Rust API generates bindings for all languages
2. **Type Safety**: Automatic memory management and type checking
3. **Better Error Handling**: Proper Result types instead of error codes
4. **Cleaner API**: No manual pointer management in Rust code
5. **Documentation**: Comments are automatically included in generated bindings
6. **Maintainability**: Much easier to add new functions and types

## Breaking Changes

1. **String handling**: All strings now use UTF-8 by default (was C-style null-terminated)
2. **Error handling**: Functions return Result types instead of custom Result enum
3. **Memory management**: Opaque types instead of #[repr(C)] structs
4. **API naming**: Some functions renamed to follow Rust conventions

## Resources

- [Diplomat Documentation](https://rust-diplomat.github.io/diplomat/)
- [Diplomat GitHub](https://github.com/rust-diplomat/diplomat)
- [Example controller.c project](../controller.c) - Reference implementation

