#!/bin/bash

# Script to build Dojo C bindings for Android targets
# Usage: ./scripts/build_android.sh [target] [--release]

set -e

# Default values
TARGET=""
BUILD_MODE="--release"
TARGETS=(
    "aarch64-linux-android"
    "armv7-linux-androideabi"
    "i686-linux-android"
    "x86_64-linux-android"
)

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --debug) BUILD_MODE="" ;;
        aarch64-linux-android|armv7-linux-androideabi|i686-linux-android|x86_64-linux-android)
            TARGET="$1"
            ;;
        -h|--help)
            echo "Usage: $0 [target] [--release|--debug]"
            echo ""
            echo "Available targets:"
            echo "  aarch64-linux-android       (ARM64)"
            echo "  armv7-linux-androideabi     (ARMv7)"
            echo "  i686-linux-android          (x86)"
            echo "  x86_64-linux-android        (x86_64)"
            echo ""
            echo "If no target is specified, all targets will be built."
            exit 0
            ;;
        *)
            echo "Unknown argument: $1"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
    esac
    shift
done

# Check if cross is installed
if ! command -v cross &> /dev/null; then
    echo "Error: 'cross' is not installed."
    echo "Install it with: cargo install cross --git https://github.com/cross-rs/cross"
    exit 1
fi

# Function to build for a specific target
build_target() {
    local target=$1
    echo ""
    echo "=========================================="
    echo "Building for target: $target"
    echo "Build mode: ${BUILD_MODE:-debug}"
    echo "=========================================="
    echo ""
    
    # Add the target
    rustup target add "$target" 2>/dev/null || true
    
    # Build using cross
    cross build --target "$target" $BUILD_MODE -p c
    
    echo ""
    echo "✓ Build completed for $target"
    
    # Show the output location
    if [ -n "$BUILD_MODE" ]; then
        OUTPUT_DIR="target/$target/release"
    else
        OUTPUT_DIR="target/$target/debug"
    fi
    
    if [ -f "$OUTPUT_DIR/libc.so" ]; then
        echo "  Output: $OUTPUT_DIR/libc.so"
        ls -lh "$OUTPUT_DIR/libc.so"
    fi
}

# Build for specified target or all targets
if [ -n "$TARGET" ]; then
    build_target "$TARGET"
else
    echo "Building for all Android targets..."
    for target in "${TARGETS[@]}"; do
        build_target "$target"
    done
    echo ""
    echo "=========================================="
    echo "All Android builds completed!"
    echo "=========================================="
fi

