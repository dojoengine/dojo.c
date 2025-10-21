#!/bin/bash

# Default to not rebuilding
REBUILD=0
FORWARD_ARGS=()

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -r|--rebuild) REBUILD=1 ;;
        *) FORWARD_ARGS+=("$1") ;;  # Store unrecognized args to forward
    esac
    shift
done

# Build the Rust library in release mode if requested
if [ $REBUILD -eq 1 ]; then
    echo "Building Rust library in release mode..."
    cargo build --release -p dojo-c
fi

# Check if the dylib/so was built successfully
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_PATH="./target/release/libdojo_c.dylib"
else
    LIB_PATH="./target/release/libdojo_c.so"
fi

if [ ! -f "$LIB_PATH" ]; then
    echo "Rust library not found at $LIB_PATH. Building..."
    cargo build --release -p c
    if [ ! -f "$LIB_PATH" ]; then
        echo "Failed to build Rust library"
        exit 1
    fi
fi

# Compile the C program
echo "Compiling C program..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    clang -o example/main example/main.c -I./crates/c -L./target/release -lc -Wl,-rpath,./target/release
else
    clang -o example/main example/main.c -I./crates/c -L./target/release -lc -Wl,-rpath=$PWD/target/release
fi

if [ $? -ne 0 ]; then
    echo "Failed to compile C program"
    exit 1
fi

# Run the program with forwarded arguments
echo "Running program..."
./example/main "${FORWARD_ARGS[@]}"