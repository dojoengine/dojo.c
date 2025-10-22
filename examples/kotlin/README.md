# Dojo Kotlin Examples

This directory contains example code for Kotlin bindings (currently non-functional).

## 🚧 Current Status: Not Working

The Kotlin bindings **do not currently compile** due to limitations in UniFFI v0.30's Kotlin generator when handling:
- Complex recursive types (`Clause`, `Primitive`, `Ty`, etc.)
- Generic types with custom converters
- Nested enum variants

The generated `dojo.kt` file itself has type errors that prevent compilation. This is a known limitation of UniFFI's Kotlin backend, not an issue with our implementation.

## ✅ Working Alternatives

**For full functionality, please use:**
- ✅ **Swift bindings** (`examples/swift/`) - Fully functional
- ✅ **Python bindings** (`examples/python/`) - Fully functional

Both Swift and Python bindings work perfectly with the synchronous client and all query types.

## Future Support

Kotlin bindings will become functional when:
1. UniFFI's Kotlin generator improves support for complex types (expected in future versions)
2. OR we simplify the type system (would limit functionality)

## Prerequisites

- **Kotlin**: Install via Homebrew:
  ```bash
  brew install kotlin
  ```

- **Dojo Library**: Build the Dojo library:
  ```bash
  cargo build --release -p dojo-uniffi
  ```

- **Kotlin Bindings**: Generate the Kotlin bindings:
  ```bash
  ./target/release/uniffi-bindgen-kotlin
  ```

## Examples

### Simple Test

Basic test to verify Kotlin bindings work.

**Run:**
```bash
chmod +x examples/kotlin/run_simple_test.sh
./examples/kotlin/run_simple_test.sh
```

**With custom Torii URL:**
```bash
./examples/kotlin/run_simple_test.sh http://localhost:8080
```

**Features demonstrated:**
- Creating a Torii client connection
- Basic error handling

### Fetch Entities (Limited)

Full query example - currently has type generation issues with complex recursive types.

```bash
./examples/kotlin/run_fetch_entities.sh
```

Note: This example demonstrates the intended API but won't compile until UniFFI's Kotlin generator better supports recursive types.

## Project Structure

```
examples/kotlin/
├── FetchEntities.kt           # Fetch entities example
├── run_fetch_entities.sh      # Build and run script
└── README.md                  # This file
```

## How It Works

1. The run script copies the Kotlin bindings and native library to a build directory
2. Compiles all Kotlin files into a JAR
3. Sets up the library path for the native library (libdojo_uniffi.dylib)
4. Runs the example with the specified parameters

## Code Example

```kotlin
import com.dojoengine.dojo.*

// Create a client
val client = ToriiClient("http://localhost:8080")

// Create a query
val query = Query(
    worldAddresses = emptyList(),
    pagination = Pagination(
        cursor = null,
        limit = 10u,
        direction = PaginationDirection.FORWARD,
        orderBy = emptyList()
    ),
    clause = null,
    noHashedKeys = false,
    models = emptyList(),
    historical = false
)

// Fetch entities
val page = client.entities(query)

// Process results
page.items.forEach { entity ->
    println("Entity: ${entity.hashedKeys}")
}
```

## Error Handling

All Dojo operations can throw `DojoException`:

```kotlin
try {
    val client = ToriiClient(toriiUrl)
    val page = client.entities(query)
} catch (e: DojoException) {
    println("Error: ${e.message}")
}
```

## Building Manually

If you prefer to compile manually:

```bash
cd examples/kotlin
mkdir -p build
cp -r ../../bindings/kotlin/* build/
cp ../../target/release/libdojo_uniffi.dylib build/
cp FetchEntities.kt build/
cd build
kotlinc *.kt -include-runtime -d FetchEntities.jar
java -Djava.library.path=. -jar FetchEntities.jar
```

## Troubleshooting

**"kotlinc not found"**
- Install Kotlin: `brew install kotlin`

**"Library not found"**
- Build the library: `cargo build --release -p dojo-uniffi`

**"Kotlin bindings not found"**
- Generate bindings: `./target/release/uniffi-bindgen-kotlin`

**"java.lang.UnsatisfiedLinkError"**
- Make sure the native library is in the same directory as the JAR
- Check that `DYLD_LIBRARY_PATH` is set correctly (done automatically by the run script)

## Additional Resources

- [Dojo Book](https://book.dojoengine.org/)
- [Kotlin Documentation](https://kotlinlang.org/docs/home.html)
- [Torii Documentation](https://book.dojoengine.org/toolchain/torii/)

