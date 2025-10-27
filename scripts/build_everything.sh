#!/bin/bash
# Master script to build ALL Dojo bindings (UniFFI, C, and WASM)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

echo "========================================"
echo "Building ALL Dojo Bindings"
echo "========================================"
echo ""

# Build UniFFI bindings (Swift, Kotlin, Python, C#, Go)
echo "🔧 Building UniFFI bindings..."
./scripts/build_all_bindings.sh
echo ""

# Build C bindings
echo "🔧 Building C bindings..."
./scripts/build_c.sh
echo ""

# Build C++ bindings
echo "🔧 Building C++ bindings..."
./scripts/build_cpp.sh
echo ""

# Build WASM bindings
echo "🔧 Building WASM bindings..."
./scripts/build_wasm.sh
echo ""

echo "✅ ALL BINDINGS GENERATED SUCCESSFULLY!"
echo ""
echo "Generated bindings:"
echo "  📁 UniFFI bindings:"
echo "     - bindings/swift/"
echo "     - bindings/kotlin/"
echo "     - bindings/python/"
echo "     - bindings/csharp/"
echo "     - bindings/go/"
echo "  📁 C/C++ bindings:"
echo "     - bindings/c/dojo.h"
echo "     - bindings/cpp/"
echo "  📁 WASM bindings:"
echo "     - pkg/"
echo ""

