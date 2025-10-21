#!/bin/bash

# Script to compile and run the Kotlin fetch_entities example
# Usage: ./run_fetch_entities.sh [torii_url] [world_address]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
TORII_URL="${1:-http://localhost:8080}"
WORLD_ADDRESS="${2:-0x0}"

echo "Running Kotlin example..."
echo "Torii URL: $TORII_URL"
echo "World Address: $WORLD_ADDRESS"
echo

# Get the repository root (3 levels up from this script)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Paths
BINDINGS_DIR="$REPO_ROOT/bindings/kotlin"
LIB_PATH="$REPO_ROOT/target/release/libdojo_uniffi.dylib"
EXAMPLE_FILE="$SCRIPT_DIR/FetchEntities.kt"

# Check if library exists
if [ ! -f "$LIB_PATH" ]; then
    echo -e "${RED}Error: Library not found at $LIB_PATH${NC}"
    echo "Please build the library first with:"
    echo "  cargo build --release -p dojo-uniffi"
    exit 1
fi

# Check if Kotlin bindings exist
if [ ! -d "$BINDINGS_DIR" ]; then
    echo -e "${RED}Error: Kotlin bindings not found at $BINDINGS_DIR${NC}"
    echo "Please generate the bindings first with:"
    echo "  ./target/release/uniffi-bindgen-kotlin"
    exit 1
fi

# Check if kotlinc is installed
if ! command -v kotlinc &> /dev/null; then
    echo -e "${RED}Error: kotlinc not found${NC}"
    echo "Please install Kotlin:"
    echo "  macOS:   brew install kotlin"
    echo "  Linux:   See https://kotlinlang.org/docs/command-line.html"
    exit 1
fi

# Create temporary build directory
BUILD_DIR="$SCRIPT_DIR/build"
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

# Copy the Kotlin bindings
echo "Copying Kotlin bindings..."
cp -r "$BINDINGS_DIR"/* "$BUILD_DIR/"

# Copy the example file
cp "$EXAMPLE_FILE" "$BUILD_DIR/"

# Copy the library
echo "Copying native library..."
cp "$LIB_PATH" "$BUILD_DIR/"

# Compile the Kotlin code
echo "Compiling Kotlin code..."
cd "$BUILD_DIR"

# Find all Kotlin files
KOTLIN_FILES=$(find . -name "*.kt")

# Compile with JNA on classpath
kotlinc $KOTLIN_FILES -include-runtime -cp "$JNA_JAR" -d FetchEntities.jar

if [ $? -ne 0 ]; then
    echo -e "${RED}Compilation failed${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Compilation successful${NC}"
echo

# Set library path for macOS
export DYLD_LIBRARY_PATH="$BUILD_DIR:$DYLD_LIBRARY_PATH"

# Run the example
echo "Running example..."
echo
java -Djava.library.path="$BUILD_DIR" -cp "$JNA_JAR:FetchEntities.jar" com.dojoengine.examples.FetchEntitiesKt "$TORII_URL" "$WORLD_ADDRESS"

# Cleanup on success
cd "$SCRIPT_DIR"
# Uncomment to keep build artifacts for inspection:
# echo "Build artifacts kept in: $BUILD_DIR"

