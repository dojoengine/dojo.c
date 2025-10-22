#!/bin/bash
# Script to build all language bindings

set -e

echo "Building All Dojo Bindings"
echo "=========================="
echo ""

# Build the uniffi library first
echo "Step 1: Building Rust library..."
cargo build --release -p dojo-uniffi
echo "✓ Library built"
echo ""

# Build all bindgen tools
echo "Step 2: Building bindgen tools..."
cargo build --release -p dojo-uniffi \
    --bin uniffi-bindgen-swift \
    --bin uniffi-bindgen-kotlin \
    --bin uniffi-bindgen-python \
    --bin uniffi-bindgen-csharp \
    --bin uniffi-bindgen-go
echo "✓ All bindgen tools built"
echo ""

# Generate Swift bindings
echo "Step 3: Generating Swift bindings..."
./target/release/uniffi-bindgen-swift
echo "✓ Swift bindings generated"
echo ""

# Generate Kotlin bindings
echo "Step 4: Generating Kotlin bindings..."
./target/release/uniffi-bindgen-kotlin
echo "✓ Kotlin bindings generated"
echo ""

# Generate Python bindings
echo "Step 5: Generating Python bindings..."
./target/release/uniffi-bindgen-python
echo "✓ Python bindings generated"
echo ""

# Generate C# bindings (if uniffi-bindgen-cs is available)
echo "Step 6: Generating C# bindings..."
if command -v uniffi-bindgen-cs >/dev/null 2>&1; then
    ./target/release/uniffi-bindgen-csharp
    echo "✓ C# bindings generated"
else
    echo "⚠ Skipped: uniffi-bindgen-cs not installed"
    echo "  Install with: cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.10.0+v0.29.4"
fi
echo ""

# Generate Go bindings (if uniffi-bindgen-go is available)
echo "Step 7: Generating Go bindings..."
if command -v uniffi-bindgen-go >/dev/null 2>&1; then
    ./target/release/uniffi-bindgen-go
    echo "✓ Go bindings generated"
else
    echo "⚠ Skipped: uniffi-bindgen-go not installed"
    echo "  Install with: cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.4.0+v0.28.3"
fi
echo ""

echo "✅ All available bindings generated!"
echo ""
echo "Generated bindings in:"
echo "  - bindings/swift/"
echo "  - bindings/kotlin/"
echo "  - bindings/python/"
if command -v uniffi-bindgen-cs >/dev/null 2>&1; then
    echo "  - bindings/csharp/"
fi
if command -v uniffi-bindgen-go >/dev/null 2>&1; then
    echo "  - bindings/go/"
fi
echo ""
echo "To install external tools, run: ./scripts/install_bindgen_tools.sh"

