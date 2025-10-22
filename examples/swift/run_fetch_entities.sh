#!/bin/bash
set -e

# Get the repository root (2 levels up from this script)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$REPO_ROOT"

# Check if library exists
if [ ! -f "target/release/libdojo_uniffi.dylib" ]; then
    echo "Error: Library not found. Please build it first:"
    echo "  cargo build --release"
    exit 1
fi

# Check if Swift bindings exist
if [ ! -f "bindings/swift/DojoEngine.swift" ]; then
    echo "Error: Swift bindings not found. Please generate them first:"
    echo "  cargo run --bin uniffi-bindgen-swift --release -- target/release/libdojo_uniffi.dylib bindings/swift --swift-sources --headers --modulemap"
    exit 1
fi

if [ ! -f "bindings/swift/DojoEngineFFI.h" ]; then
    echo "Error: Swift binding headers not found. Regenerating..."
    cargo run --bin uniffi-bindgen-swift --release -- target/release/libdojo_uniffi.dylib bindings/swift --swift-sources --headers --modulemap
fi

# Parse arguments
TORII_URL="${1:-http://localhost:8080}"
WORLD_ADDRESS="${2:-0x0}"

# Create a temporary directory for compilation
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Copy all necessary files
cp "bindings/swift/DojoEngine.swift" "$TEMP_DIR/"
cp "bindings/swift/DojoEngineFFI.h" "$TEMP_DIR/"
cp "bindings/swift/dojo_uniffi.modulemap" "$TEMP_DIR/module.modulemap"
cp "examples/swift/fetch_entities.swift" "$TEMP_DIR/"

# Create a combined Swift file
cat > "$TEMP_DIR/main.swift" << 'EOF'
// Combined Swift bindings and example
EOF

cat "bindings/swift/DojoEngine.swift" >> "$TEMP_DIR/main.swift"

cat >> "$TEMP_DIR/main.swift" << 'EOF'

// Example code below
EOF

# Add the example code (skip the first 10 lines which are comments and import)
tail -n +10 "examples/swift/fetch_entities.swift" >> "$TEMP_DIR/main.swift"

echo "Running Swift example..."
echo "Torii URL: $TORII_URL"
echo "World Address: $WORLD_ADDRESS"
echo ""

# Compile with proper module import and bridging header
cd "$TEMP_DIR"
swiftc -o fetch_entities \
    -import-objc-header DojoEngineFFI.h \
    -I . \
    -L "$REPO_ROOT/target/release" \
    -ldojo_uniffi \
    -Xlinker -rpath -Xlinker "$REPO_ROOT/target/release" \
    main.swift

# Run the compiled program
DYLD_LIBRARY_PATH="$REPO_ROOT/target/release" \
    ./fetch_entities "$TORII_URL" "$WORLD_ADDRESS"

