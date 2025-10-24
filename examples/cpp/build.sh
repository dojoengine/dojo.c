#!/bin/bash

set -e

echo "🔨 Building Dojo C++ Examples..."
echo ""

# Build the library first
cd ../..
echo "📦 Building dojo-uniffi library..."
cargo build --release -p dojo-uniffi
echo "✓ Library built"
echo ""

# Go to examples/cpp
cd examples/cpp

# Create build directory
mkdir -p build
cd build

# Configure and build
echo "⚙️  Configuring CMake..."
cmake ..
echo ""

echo "🔧 Compiling C++ examples..."
cmake --build .
echo "✓ Compiled successfully"
echo ""

echo "✅ Build complete!"
echo ""
echo "Run the example with:"
echo "  ./build/fetch_entities [torii_url] [world_address]"
echo ""
echo "Example:"
echo "  ./build/fetch_entities http://localhost:8080 0x1234..."
echo ""


