#!/bin/bash

# Script to build Dojo C bindings for native platform
# Usage: ./scripts/build_c.sh [--release|--debug] [target]

set -e

# Default values
BUILD_MODE="--release"
TARGET=""

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --debug) BUILD_MODE="" ;;
        --release) BUILD_MODE="--release" ;;
        -h|--help)
            echo "Usage: $0 [--release|--debug] [target]"
            echo ""
            echo "Options:"
            echo "  --release    Build in release mode (default)"
            echo "  --debug      Build in debug mode"
            echo ""
            echo "Examples:"
            echo "  $0                                  # Build for current platform in release mode"
            echo "  $0 --debug                          # Build for current platform in debug mode"
            echo "  $0 x86_64-apple-darwin              # Build for specific target"
            echo "  $0 --debug aarch64-apple-darwin     # Build for specific target in debug mode"
            exit 0
            ;;
        *)
            TARGET="$1"
            ;;
    esac
    shift
done

# Determine output info
if [ -n "$BUILD_MODE" ]; then
    BUILD_DIR="release"
else
    BUILD_DIR="debug"
fi

echo "=========================================="
echo "Building Dojo C Bindings"
echo "=========================================="
echo "Build mode: ${BUILD_MODE:-debug}"
if [ -n "$TARGET" ]; then
    echo "Target: $TARGET"
else
    echo "Target: native ($(rustc -vV | grep host | cut -d' ' -f2))"
fi
echo "=========================================="
echo ""

# Build command
if [ -n "$TARGET" ]; then
    rustup target add "$TARGET" 2>/dev/null || true
    cargo build $BUILD_MODE --target "$TARGET" -p c
    OUTPUT_DIR="target/$TARGET/$BUILD_DIR"
else
    cargo build $BUILD_MODE -p c
    OUTPUT_DIR="target/$BUILD_DIR"
fi

echo ""
echo "✅ Build completed successfully!"
echo ""

# Determine library file based on OS
if [ -f "$OUTPUT_DIR/libc.dylib" ]; then
    LIB_FILE="libc.dylib"
elif [ -f "$OUTPUT_DIR/libc.so" ]; then
    LIB_FILE="libc.so"
elif [ -f "$OUTPUT_DIR/c.dll" ]; then
    LIB_FILE="c.dll"
elif [ -f "$OUTPUT_DIR/libc.a" ]; then
    LIB_FILE="libc.a"
else
    echo "⚠️  Warning: Could not find output library file"
    echo "Output directory: $OUTPUT_DIR"
    exit 0
fi

echo "📦 Output files:"
echo "   Header: crates/c/dojo.h"
echo "   Library: $OUTPUT_DIR/$LIB_FILE"
echo ""

# Show file size
if command -v ls &> /dev/null; then
    ls -lh "$OUTPUT_DIR/$LIB_FILE" | awk '{print "   Size: " $5}'
fi

echo ""
echo "To use the library:"
echo "   Include: -I./crates/c"
echo "   Link: -L./$OUTPUT_DIR -lc"

