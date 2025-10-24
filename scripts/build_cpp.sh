#!/bin/bash

# Build C++ bindings for Dojo
# This script builds the dojo library and generates C++ bindings

set -e

echo "🔧 Building Dojo C++ Bindings"
echo "===================================="

# Colors for output
GREEN='\033[0.32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Note: We use the installed uniffi-bindgen-cpp binary
echo "✓ Using installed uniffi-bindgen-cpp binary"

# Build the library
echo ""
echo "📦 Building dojo library..."
cd "$(dirname "$0")/.."
cargo build --release -p dojo-uniffi

if [ $? -ne 0 ]; then
    echo "❌ Failed to build library"
    exit 1
fi

echo "✓ Library built successfully"

# Generate C++ bindings
echo ""
echo "🔨 Generating C++ bindings..."
cd crates/uniffi

cargo run --bin uniffi-bindgen-cpp --release

if [ $? -ne 0 ]; then
    echo "❌ Failed to generate C++ bindings"
    exit 1
fi

echo ""
echo -e "${GREEN}✅ C++ bindings generated successfully!${NC}"
echo ""
echo "📁 Generated files:"
echo "   ../../bindings/cpp/dojo.hpp"
echo "   ../../bindings/cpp/dojo.cpp"
echo "   ../../bindings/cpp/dojo_scaffolding.hpp"
echo ""
echo "📚 Library location:"
echo "   ../../target/release/libdojo_uniffi.dylib (macOS)"
echo "   ../../target/release/libdojo_uniffi.so (Linux)"
echo "   ../../target/release/dojo_uniffi.dll (Windows)"
echo ""
echo "To use in your C++ project:"
echo "  1. Include dojo.hpp"
echo "  2. Link against libdojo_uniffi"
echo "  3. Ensure the dylib/so/dll is in your library path"
echo ""
echo "Example CMakeLists.txt:"
echo "  add_executable(my_app main.cpp dojo.cpp)"
echo "  target_include_directories(my_app PRIVATE bindings/cpp)"
echo "  target_link_libraries(my_app \${CMAKE_SOURCE_DIR}/target/release/libdojo_uniffi.dylib)"
echo ""


