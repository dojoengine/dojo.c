#!/bin/bash
set -e

# Script to build Go bindings for Dojo
# Usage: ./scripts/build_go.sh

echo "Building Dojo Go bindings..."
echo ""

# Step 1: Build the uniffi library
echo "1. Building uniffi library..."
cargo build --release -p dojo-uniffi
echo "✓ Library built"
echo ""

# Step 2: Build the bindgen tool
echo "2. Building Go bindgen tool..."
cargo build --release --bin uniffi-bindgen-go -p dojo-uniffi
echo "✓ Bindgen tool built"
echo ""

# Step 3: Check if uniffi-bindgen-go is installed
if ! command -v uniffi-bindgen-go &> /dev/null; then
    echo "⚠ uniffi-bindgen-go is not installed"
    echo ""
    echo "Installing uniffi-bindgen-go..."
    cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.4.0+v0.28.3
    echo "✓ uniffi-bindgen-go installed"
    echo ""
fi

# Step 4: Generate bindings
echo "3. Generating Go bindings..."
./target/release/uniffi-bindgen-go
echo "✓ Go bindings generated"
echo ""

echo "✅ All done!"
echo ""
echo "Bindings are in: bindings/go/"
echo ""
echo "To use in your Go project:"
echo "  1. Copy the generated Go files to your project"
echo "  2. Set LD_LIBRARY_PATH to include target/release/"
echo "  3. Import and use the bindings in your Go code"
echo ""
echo "Example:"
echo "  export LD_LIBRARY_PATH=\$PWD/target/release:\$LD_LIBRARY_PATH"
echo "  go run examples/go/fetch_entities.go"

