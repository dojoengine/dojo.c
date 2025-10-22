#!/bin/bash
# Script to install external UniFFI binding generators

set -e

echo "Installing External UniFFI Binding Generators"
echo "=============================================="
echo ""

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check Rust/Cargo
if ! command_exists cargo; then
    echo "❌ Error: cargo not found"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "Rust/Cargo found ✓"
echo ""

# Install uniffi-bindgen-cs
echo "1. Installing uniffi-bindgen-cs..."
if command_exists uniffi-bindgen-cs; then
    echo "   uniffi-bindgen-cs is already installed"
    uniffi-bindgen-cs --version || echo "   (version info not available)"
else
    echo "   Installing from GitHub..."
    cargo install uniffi-bindgen-cs \
        --git https://github.com/NordSecurity/uniffi-bindgen-cs \
        --tag v0.10.0+v0.29.4
    echo "   ✓ uniffi-bindgen-cs installed"
fi
echo ""

# Install uniffi-bindgen-go
echo "2. Installing uniffi-bindgen-go..."
if command_exists uniffi-bindgen-go; then
    echo "   uniffi-bindgen-go is already installed"
    uniffi-bindgen-go --version 2>/dev/null || echo "   (version info not available)"
else
    echo "   Installing from GitHub..."
    cargo install uniffi-bindgen-go \
        --git https://github.com/NordSecurity/uniffi-bindgen-go \
        --tag v0.4.0+v0.28.3
    echo "   ✓ uniffi-bindgen-go installed"
fi
echo ""

echo "✅ All external binding generators installed!"
echo ""
echo "Installed tools:"
echo "  - uniffi-bindgen-cs  (for C# bindings)"
echo "  - uniffi-bindgen-go  (for Go bindings)"
echo ""
echo "You can now generate bindings with:"
echo "  ./scripts/build_csharp.sh"
echo "  ./scripts/build_go.sh"
echo ""
echo "Or generate all bindings with:"
echo "  ./scripts/build_all_bindings.sh"

