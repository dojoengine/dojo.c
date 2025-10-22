# Dojo Go Examples

This directory contains examples demonstrating how to use the Dojo Go bindings.

## Prerequisites

- Go 1.19 or higher
- Dojo uniffi library built (`cargo build --release -p dojo-uniffi`)
- Generated Go bindings (see below)

## Generating Bindings

Before running the examples, you need to generate the Go bindings:

```bash
# From the repository root
./scripts/build_go.sh
```

This will:
1. Build the Rust library
2. Install `uniffi-bindgen-go` if not already installed
3. Generate Go bindings in `bindings/go/`

## Setting Up Your Environment

The compiled Rust library must be accessible to your Go application:

### macOS
```bash
export DYLD_LIBRARY_PATH=$PWD/../../target/release:$DYLD_LIBRARY_PATH
```

### Linux
```bash
export LD_LIBRARY_PATH=$PWD/../../target/release:$LD_LIBRARY_PATH
```

### Windows
Add the directory containing `dojo_uniffi.dll` to your PATH, or copy the DLL to your executable's directory.

## Running the Examples

### Fetch Entities Example

This example shows how to connect to a Torii server and fetch entities.

```bash
# Navigate to the examples directory
cd examples/go

# Set up the library path (macOS example)
export DYLD_LIBRARY_PATH=$PWD/../../target/release:$DYLD_LIBRARY_PATH

# Run the example
go run fetch_entities.go

# Or with custom Torii URL and world address
go run fetch_entities.go http://localhost:8080 0x1234...
```

## Using in Your Own Project

1. **Set up your Go module**:
   ```bash
   go mod init your-project
   ```

2. **Copy or reference the generated bindings**:
   ```bash
   # Option 1: Copy the bindings
   cp -r ../../bindings/go/uniffi ./
   
   # Option 2: Use go.mod replace directive
   # In go.mod:
   replace dojo => /path/to/dojo.c/bindings/go
   ```

3. **Import and use the bindings**:
   ```go
   import dojo "your-project/uniffi/dojo"
   
   client, err := dojo.ToriiClientNewWithConfig("http://localhost:8080", 4*1024*1024)
   if err != nil {
       log.Fatal(err)
   }
   defer client.Destroy()
   
   page, err := client.Entities(query)
   // ... use the results
   ```

4. **Build with proper library paths**:
   ```bash
   # Make sure LD_LIBRARY_PATH/DYLD_LIBRARY_PATH is set
   go build
   ```

## Project Structure

When uniffi-bindgen-go generates bindings, it typically creates:

```
uniffi/
  dojo/
    dojo.go         # Generated Go bindings
    dojo_test.go    # Generated tests (if any)
```

Adjust your import paths based on where you place these files.

## Building a Standalone Binary

To create a binary that can find the native library:

### Option 1: Set RPATH (Linux/macOS)
```bash
go build -ldflags "-r /path/to/dojo.c/target/release" fetch_entities.go
```

### Option 2: Static Linking
For static linking, you'll need to build the Rust library with:
```bash
cargo build --release -p dojo-uniffi --features static
```
(Note: This may require additional configuration)

### Option 3: Bundle the Library
Copy the native library alongside your executable:
```bash
go build fetch_entities.go
cp ../../target/release/libdojo_uniffi.dylib .  # macOS
# or
cp ../../target/release/libdojo_uniffi.so .     # Linux
```

## Troubleshooting

### "library not found" errors

The most common issue is that Go can't find the native library. Solutions:

1. **Set the library path**:
   - macOS: `export DYLD_LIBRARY_PATH=/path/to/target/release`
   - Linux: `export LD_LIBRARY_PATH=/path/to/target/release`
   - Windows: Add to PATH or copy DLL to exe directory

2. **Verify the library exists**:
   ```bash
   ls -la ../../target/release/libdojo_uniffi.*
   ```

3. **Check library dependencies** (macOS):
   ```bash
   otool -L ../../target/release/libdojo_uniffi.dylib
   ```

4. **Check library dependencies** (Linux):
   ```bash
   ldd ../../target/release/libdojo_uniffi.so
   ```

### Import errors

Make sure your import paths match where the bindings are generated:
- Check the `go.mod` file for correct replace directives
- Verify the package structure matches your imports

### Compilation errors

- Ensure you're using Go 1.19 or higher
- Run `go mod tidy` to clean up dependencies
- Check that the bindings were generated successfully

### Runtime errors

- Make sure the Torii server is running and accessible
- Check that the world address is correct
- Verify network connectivity

## Cross-Compilation Notes

When cross-compiling, you'll need:
1. The native library compiled for the target platform
2. CGO enabled: `CGO_ENABLED=1`
3. Appropriate C compiler for the target platform

Example for cross-compiling to Linux from macOS:
```bash
CGO_ENABLED=1 GOOS=linux GOARCH=amd64 go build
```

## Available Examples

- **fetch_entities.go** - Demonstrates connecting to Torii and fetching entities with pagination

## Script Helper

You can create a script to make running easier:

```bash
#!/bin/bash
# run.sh

export DYLD_LIBRARY_PATH=$PWD/../../target/release:$DYLD_LIBRARY_PATH  # macOS
export LD_LIBRARY_PATH=$PWD/../../target/release:$LD_LIBRARY_PATH      # Linux

go run "$@"
```

Usage:
```bash
chmod +x run.sh
./run.sh fetch_entities.go
```

## More Information

For more details on the Dojo Go bindings:
- UniFFI Go bindings: https://github.com/NordSecurity/uniffi-bindgen-go
- Dojo documentation: https://book.dojoengine.org/

