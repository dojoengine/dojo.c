#!/bin/bash

# Script to run the simple Kotlin test
# Usage: ./run_simple_test.sh [torii_url]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Default values
TORII_URL="${1:-http://localhost:8080}"

echo "Running Kotlin simple test..."
echo "Torii URL: $TORII_URL"
echo

# Get the repository root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Paths
BINDINGS_DIR="$REPO_ROOT/bindings/kotlin"
LIB_PATH="$REPO_ROOT/target/release/libdojo_uniffi.dylib"
TEST_FILE="$SCRIPT_DIR/SimpleTest.kt"

# Check if library exists
if [ ! -f "$LIB_PATH" ]; then
    echo -e "${RED}Error: Library not found at $LIB_PATH${NC}"
    echo "Please build the library first with:"
    echo "  cargo build --release -p dojo-uniffi"
    exit 1
fi

# Check if kotlinc is installed
if ! command -v kotlinc &> /dev/null; then
    echo -e "${RED}Error: kotlinc not found${NC}"
    echo "Please install Kotlin:"
    echo "  macOS:   brew install kotlin"
    exit 1
fi

# Create temporary build directory
BUILD_DIR="$SCRIPT_DIR/build-simple"
mkdir -p "$BUILD_DIR"

# Download JNA if not present
JNA_VERSION="5.13.0"
JNA_JAR="$BUILD_DIR/jna-$JNA_VERSION.jar"

if [ ! -f "$JNA_JAR" ]; then
    echo "Downloading JNA library..."
    curl -L -o "$JNA_JAR" "https://repo1.maven.org/maven2/net/java/dev/jna/jna/$JNA_VERSION/jna-$JNA_VERSION.jar"
    if [ $? -ne 0 ]; then
        echo -e "${RED}Failed to download JNA library${NC}"
        exit 1
    fi
fi

# Copy just the core Kotlin binding file (avoid complex types)
echo "Copying Kotlin bindings..."
cp "$BINDINGS_DIR/com/dojoengine/dojo/dojo.kt" "$BUILD_DIR/" 2>/dev/null || {
    mkdir -p "$BUILD_DIR/com/dojoengine/dojo"
    cp "$BINDINGS_DIR/com/dojoengine/dojo/dojo.kt" "$BUILD_DIR/com/dojoengine/dojo/"
}

# Copy the test file
cp "$TEST_FILE" "$BUILD_DIR/"

# Copy the library
echo "Copying native library..."
cp "$LIB_PATH" "$BUILD_DIR/"

# Compile just the simple test
echo "Compiling Kotlin code..."
cd "$BUILD_DIR"

# Try to compile
kotlinc SimpleTest.kt -include-runtime -cp "$JNA_JAR" -d SimpleTest.jar 2>&1 | head -20

if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo -e "${RED}Compilation failed${NC}"
    echo ""
    echo "Note: The Kotlin bindings have limitations with complex recursive types."
    echo "This is a known issue with UniFFI's Kotlin generator."
    echo ""
    echo "For full functionality, please use:"
    echo "  - Swift bindings (examples/swift/)"
    echo "  - Python bindings (examples/python/)"
    echo ""
    exit 1
fi

echo -e "${GREEN}✓ Compilation successful${NC}"
echo

# Set library path
export DYLD_LIBRARY_PATH="$BUILD_DIR:$DYLD_LIBRARY_PATH"

# Run the test
echo "Running test..."
echo
java -Djava.library.path="$BUILD_DIR" -cp "$JNA_JAR:SimpleTest.jar" com.dojoengine.examples.SimpleTestKt "$TORII_URL"

cd "$SCRIPT_DIR"

