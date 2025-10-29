#!/bin/bash
set -e

echo "Building Swift bindings for dojo-uniffi..."

# Build the Rust library
cargo build --release -p dojo-uniffi

# Generate Swift bindings
cargo run --release --bin uniffi-bindgen-swift

echo "Swift bindings generated in bindings/swift/"

