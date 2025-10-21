#!/usr/bin/env python3
"""
Example: Fetch entities from a Torii server

This example demonstrates how to use the Dojo Python bindings to:
1. Connect to a Torii server
2. Query entities with pagination
3. Display entity data
"""

import sys
import shutil
from pathlib import Path
import asyncio

# Setup paths
repo_root = Path(__file__).parent.parent.parent
bindings_path = repo_root / "bindings" / "python"
lib_source = repo_root / "target" / "release" / "libdojo_uniffi.dylib"
lib_dest = bindings_path / "libdojo_uniffi.dylib"

# Copy library to bindings directory if it doesn't exist or is outdated
if not lib_dest.exists() or lib_source.stat().st_mtime > lib_dest.stat().st_mtime:
    print(f"Copying library from {lib_source} to {lib_dest}...")
    shutil.copy2(lib_source, lib_dest)
    print("✓ Library copied")

# Add the bindings directory to the path
sys.path.insert(0, str(bindings_path))

from dojo import (
    ToriiClient,
    Query,
    Pagination,
    PaginationDirection,
    OrderBy,
    OrderDirection,
)


async def fetch_entities(torii_url: str, world_address: str):
    """Fetch entities from a Torii server"""
    
    print(f"Connecting to Torii at {torii_url}...")
    
    # Create a client (using constructor without config for default 4MB max message size)
    # If you need custom config, use: ToriiClient.new_with_config(torii_url, max_message_size)
    client = await ToriiClient.new_with_config(torii_url, 4 * 1024 * 1024)
    
    print("✓ Connected successfully!")
    
    # Create a query to fetch entities
    query = Query(
        world_addresses=[],
        pagination=Pagination(
            cursor=None,  # Start from beginning
            limit=10,     # Fetch 10 entities
            direction=PaginationDirection.FORWARD,
            order_by=[
                
            ]
        ),
        clause=None,  # No filtering clause
        no_hashed_keys=False,
        models=[],  # Empty means all models
        historical=False
    )
    
    print("\nFetching entities...")
    page = await client.entities(query)
    
    print(f"\n✓ Retrieved {len(page.items)} entities")
    
    if page.next_cursor:
        print(f"Next cursor available: {page.next_cursor[:20]}...")
    else:
        print("No more pages available")
    
    # Display entity information
    for i, entity in enumerate(page.items, 1):
        print(f"\n{'='*60}")
        print(f"Entity {i}:")
        print(f"  World Address: {entity.world_address}")
        print(f"  Hashed Keys:   {entity.hashed_keys}")
        print(f"  Created At:    {entity.created_at}")
        print(f"  Updated At:    {entity.updated_at}")
        print(f"  Executed At:   {entity.executed_at}")
        print(f"  Models:        {len(entity.models)} model(s)")
        
        # Display model information
        for j, model in enumerate(entity.models, 1):
            print(f"\n  Model {j}: {model.name}")
            print(f"    Children: {len(model.children)} field(s)")
            for child in model.children[:3]:  # Show first 3 fields
                print(f"      - {child.name} (key={child.key})")
            if len(model.children) > 3:
                print(f"      ... and {len(model.children) - 3} more")
    
    print(f"\n{'='*60}\n")


async def main():
    # Default Torii URL (adjust as needed)
    torii_url = "http://localhost:8080"
    
    # Example world address (adjust to your actual world address)
    world_address = "0x0"
    
    # Check for command line arguments
    if len(sys.argv) > 1:
        torii_url = sys.argv[1]
    if len(sys.argv) > 2:
        world_address = sys.argv[2]
    
    print("Dojo Python Example: Fetch Entities")
    print("====================================\n")
    
    try:
        await fetch_entities(torii_url, world_address)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        print(f"\nUsage: {sys.argv[0]} [torii_url] [world_address]")
        print(f"Example: {sys.argv[0]} http://localhost:8080 0x1234...")
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())

