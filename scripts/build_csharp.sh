#!/bin/bash
set -e

# Script to build C# bindings for Dojo
# Usage: ./scripts/build_csharp.sh

echo "Building Dojo C# bindings..."
echo ""

# Step 1: Build the uniffi library
echo "1. Building uniffi library..."
cargo build --release -p dojo-uniffi
echo "✓ Library built"
echo ""

# Step 2: Build the bindgen tool
echo "2. Building C# bindgen tool..."
cargo build --release --bin uniffi-bindgen-csharp -p dojo-uniffi
echo "✓ Bindgen tool built"
echo ""

# Step 3: Check if uniffi-bindgen-cs is installed
if ! command -v uniffi-bindgen-cs &> /dev/null; then
    echo "⚠ uniffi-bindgen-cs is not installed"
    echo ""
    echo "Installing uniffi-bindgen-cs..."
    cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.10.0+v0.29.4
    echo "✓ uniffi-bindgen-cs installed"
    echo ""
fi

# Step 4: Generate bindings
echo "3. Generating C# bindings..."
./target/release/uniffi-bindgen-csharp target/release/libdojo_uniffi.dylib bindings/csharp --no-format
echo "✓ C# bindings generated"
echo ""

echo "✅ All done!"
echo ""
echo "Bindings are in: bindings/csharp/"
echo ""
echo "To use in your C# project:"
echo "  1. Add the generated .cs file to your project"
echo "  2. Set AllowUnsafeBlocks to true in your .csproj"
echo "  3. Ensure the native library is in your library path"

