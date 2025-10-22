# dojo.c

Multi-language bindings for the Dojo Torii Client SDK, providing seamless integration across multiple platforms and languages.

## Features

This package provides multiple binding strategies for maximum compatibility:

- **C/C++ Bindings** - Generated with `cbindgen` for native applications
- **WebAssembly** - Browser and Node.js support via `wasm-bindgen`
- **UniFFI Bindings** - Modern bindings via Mozilla's UniFFI
  - Swift, Kotlin, Python (built-in)
  - C# and Go (via external generators)

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
│   ├── python/
│   ├── csharp/
│   └── go/
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

### UniFFI Bindings

Build the library first:
```bash
cargo build --release -p dojo-uniffi
```

#### Swift, Kotlin, Python (Built-in)

```bash
# Generate Swift bindings (iOS/macOS)
cargo run --bin uniffi-bindgen-swift --release -- \
    target/release/libdojo_uniffi.dylib bindings/swift --swift-sources

# Generate Kotlin bindings (Android)
cargo run --bin uniffi-bindgen-kotlin --release -- \
    target/release/libdojo_uniffi.dylib bindings/kotlin

# Generate Python bindings
cargo run --bin uniffi-bindgen-python --release -- \
    target/release/libdojo_uniffi.dylib bindings/python
```

#### C# and Go (External Generators)

C# and Go bindings use external tools. Install them first:

```bash
# Install external binding generators (one-time setup)
./scripts/install_bindgen_tools.sh
```

Then generate bindings:

```bash
# Generate C# bindings
./scripts/build_csharp.sh

# Generate Go bindings
./scripts/build_go.sh

# Or generate all bindings at once
./scripts/build_all_bindings.sh
```

**Manual installation:**

```bash
# C# (requires uniffi-bindgen-cs)
cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.10.0+v0.29.4

# Go (requires uniffi-bindgen-go)
cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.4.0+v0.28.3
```

See [`src/uniffi/README.md`](src/uniffi/README.md) for detailed UniFFI documentation.

## Documentation

- **[BINDINGS_GUIDE.md](BINDINGS_GUIDE.md)** - Comprehensive guide for all language bindings
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Quick command reference for all languages
- **[CSHARP_GO_BINDINGS_SUMMARY.md](CSHARP_GO_BINDINGS_SUMMARY.md)** - Details on C# and Go implementation

## Language Support Status

| Language | Status | Notes |
|----------|--------|-------|
| **Swift** | ✅ Fully Functional | All features working, synchronous client |
| **Python** | ✅ Fully Functional | All features working, synchronous client |
| **C#** | ✅ Functional | Via external uniffi-bindgen-cs (v0.10.0+v0.29.4) |
| **Go** | ✅ Functional | Via external uniffi-bindgen-go (v0.4.0+v0.28.3) |
| **C/C++** | ✅ Functional | Basic functionality via cbindgen |
| **WebAssembly** | ✅ Functional | Browser and Node.js support |
| **Kotlin** | 🚧 Not Working | UniFFI v0.30 limitations with complex types |

See `examples/` directory for language-specific examples and documentation.

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

See `examples/python/` for complete examples.

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

### C# Example

See `examples/csharp/` for complete examples.

```bash
cd examples/csharp
export DYLD_LIBRARY_PATH=../../target/release:$DYLD_LIBRARY_PATH  # macOS
dotnet run
```

```csharp
using uniffi.dojo;

var client = await ToriiClient.NewWithConfig("http://localhost:8080", 4 * 1024 * 1024);
var page = await client.Entities(query);
```

### Go Example

See `examples/go/` for complete examples.

```bash
cd examples/go
export DYLD_LIBRARY_PATH=../../target/release:$DYLD_LIBRARY_PATH  # macOS
go run fetch_entities.go
```

```go
import dojo "dojo/uniffi/dojo"

client, err := dojo.ToriiClientNewWithConfig("http://localhost:8080", 4*1024*1024)
if err != nil {
    log.Fatal(err)
}
defer client.Destroy()

page, err := client.Entities(query)
```
