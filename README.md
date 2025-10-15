# dojo.c

Multi-language bindings for the Dojo Torii Client SDK, providing seamless integration across multiple platforms and languages.

## Features

This package provides multiple binding strategies for maximum compatibility:

- **C/C++ Bindings** - Generated with `cbindgen` for native applications
- **WebAssembly** - Browser and Node.js support via `wasm-bindgen`
- **UniFFI Bindings** - Modern Swift, Kotlin, and Python bindings via Mozilla's UniFFI

## Project Structure

```
dojo.c/
├── src/
│   ├── c/                  # C FFI implementation
│   ├── wasm/               # WebAssembly implementation
│   ├── uniffi/             # UniFFI implementation
│   │   ├── types/          # Domain types (organized by category)
│   │   ├── client.rs       # ToriiClient with queries & subscriptions
│   │   └── README.md       # UniFFI-specific documentation
│   ├── dojo.udl            # UniFFI interface definition
│   └── lib.rs              # Main library entry point
├── src/bin/                # Binding generator binaries
├── scripts/                # Build and generation scripts
├── bindings/               # Generated bindings output
│   ├── swift/
│   ├── kotlin/
│   └── python/
└── example/                # C usage examples
```

## Building

```bash
# Build for current platform
cargo build --release
```

### Native platform

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

### Wasm

```bash
# Building wasm32 binary
cargo build --release --target wasm32-unknown-unknown
# Building using wasm-pack
cd pkg && bunx wasm-pack build --release
```

## Generating Bindings

### C/C++ Headers

Headers are automatically generated during `cargo build`:
- `dojo.h` - C header
- `dojo.hpp` - C++ header  
- `dojo.pyx` - Cython definitions

### UniFFI Bindings (Swift, Kotlin, Python)

```bash
# Build the library first
cargo build --release

# Generate Swift bindings (iOS/macOS)
cargo run --bin uniffi-bindgen-swift --release -- \
    target/release/libdojo_c.dylib bindings/swift --swift-sources

# Generate Kotlin bindings (Android)
cargo run --bin uniffi-bindgen-kotlin --release -- \
    target/release/libdojo_c.dylib bindings/kotlin

# Generate Python bindings
cargo run --bin uniffi-bindgen-python --release -- \
    target/release/libdojo_c.dylib bindings/python
```

See [`src/uniffi/README.md`](src/uniffi/README.md) for detailed UniFFI documentation.

## Running Examples

### C Example

```bash
# Build dojo.c
cargo build --release
# Compile and link C example
clang example/main.c target/release/libdojo_c.dylib
# Run example
./a.out
```

### Python Example

```python
from dojo import ToriiClient

# Create client
client = await ToriiClient("http://localhost:8080")

# Subscribe to entity updates
def on_entity_update(entity):
    print(f"Entity updated: {entity}")

def on_error(error):
    print(f"Error: {error}")

sub_id = await client.subscribe_entity_updates(
    None,  # clause
    [],    # world_addresses
    EntityUpdateCallback(on_entity_update, on_error)
)
```
