#!/bin/bash
set -e

# Get the repository root (2 levels up from this script)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$REPO_ROOT"

# Check if library exists
if [ ! -f "target/release/libdojo_c.dylib" ] && [ ! -f "target/release/libdojo_c.so" ]; then
    echo "Error: Library not found. Building it now..."
    cargo build --release -p dojo-c
fi

# Determine library extension based on platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_EXT="dylib"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    LIB_EXT="so"
else
    echo "Error: Unsupported platform: $OSTYPE"
    exit 1
fi

echo "Building C example..."

cd examples/c

gcc -o main main.c \
    -I"$REPO_ROOT" \
    -L"$REPO_ROOT/target/release" \
    -ldojo_c \
    -Wl,-rpath,"$REPO_ROOT/target/release"

echo "✓ Build successful!"
echo ""
echo "Run with: ./main"

