#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Dojo C++ Example: Fetch Entities${NC}"
echo "===================================="
echo ""

# Check if binary exists, if not build it
if [ ! -f "build/fetch_entities" ]; then
    echo -e "${YELLOW}Binary not found, building...${NC}"
    bash build.sh
    echo ""
fi

# Default values
TORII_URL="${1:-http://localhost:8080}"
WORLD_ADDRESS="${2:-0x0}"

echo "Configuration:"
echo "  Torii URL:      $TORII_URL"
echo "  World Address:  $WORLD_ADDRESS"
echo ""

# Run the example
if ./build/fetch_entities "$TORII_URL" "$WORLD_ADDRESS"; then
    echo -e "\n${GREEN}✅ Success!${NC}"
else
    EXIT_CODE=$?
    echo -e "\n${RED}❌ Failed with exit code: $EXIT_CODE${NC}"
    echo ""
    echo "Common issues:"
    echo "  • Make sure Torii is running at $TORII_URL"
    echo "  • Check that the world address is correct"
    echo "  • Verify network connectivity"
    echo ""
    echo "Usage: $0 [torii_url] [world_address]"
    echo "Example: $0 http://localhost:8080 0x1234567890abcdef"
    exit $EXIT_CODE
fi
