#!/bin/bash
# Build script for iOS XCFramework and Swift bindings for Dojo

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}=== Building Dojo iOS XCFramework ===${NC}"
echo ""

# Check for required iOS targets
echo -e "${YELLOW}Checking iOS targets...${NC}"
if ! rustup target list --installed | grep -q "aarch64-apple-ios"; then
    echo "Installing aarch64-apple-ios..."
    rustup target add aarch64-apple-ios
fi

if ! rustup target list --installed | grep -q "aarch64-apple-ios-sim"; then
    echo "Installing aarch64-apple-ios-sim..."
    rustup target add aarch64-apple-ios-sim
fi
echo -e "${GREEN}✓ iOS targets ready${NC}"
echo ""

# Step 1: Build for iOS device (arm64)
echo -e "${YELLOW}Step 1/4:${NC} Building for iOS device (aarch64-apple-ios)..."
cd "$PROJECT_ROOT"
cargo build --release -p dojo-uniffi --target aarch64-apple-ios
echo -e "${GREEN}✓ iOS device build complete${NC}"
echo ""

# Step 2: Build for iOS simulator (arm64)
echo -e "${YELLOW}Step 2/4:${NC} Building for iOS simulator (aarch64-apple-ios-sim)..."
cargo build --release -p dojo-uniffi --target aarch64-apple-ios-sim
echo -e "${GREEN}✓ iOS simulator build complete${NC}"
echo ""

# Step 3: Generate Swift bindings
echo -e "${YELLOW}Step 3/4:${NC} Generating Swift bindings..."
./scripts/build_swift.sh
echo -e "${GREEN}✓ Swift bindings generated${NC}"
echo ""

# Step 4: Create XCFramework
echo -e "${YELLOW}Step 4/4:${NC} Creating XCFramework..."
rm -rf target/dojo_uniffi.xcframework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libdojo_uniffi.a \
  -library target/aarch64-apple-ios-sim/release/libdojo_uniffi.a \
  -output target/dojo_uniffi.xcframework
echo -e "${GREEN}✓ XCFramework created${NC}"
echo ""

echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo -e "${BLUE}Summary:${NC}"
echo "  • XCFramework: target/dojo_uniffi.xcframework"
echo "  • iOS device:  target/aarch64-apple-ios/release/libdojo_uniffi.a"
echo "  • iOS sim:     target/aarch64-apple-ios-sim/release/libdojo_uniffi.a"
echo "  • Bindings:    bindings/swift/"
echo ""

