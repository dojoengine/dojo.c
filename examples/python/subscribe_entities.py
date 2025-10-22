#!/usr/bin/env python3
"""
Example: Subscribe to entity updates from a Torii server

This example demonstrates real-time entity subscriptions using callbacks.
"""

import asyncio
import sys
import shutil
from pathlib import Path

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
    EntityUpdateCallback,
)


class MyEntityCallback:
    """Custom callback handler for entity updates"""
    
    def __init__(self):
        self.update_count = 0
    
    def on_update(self, entity):
        """Called when an entity is updated"""
        self.update_count += 1
        print(f"\n📦 Entity Update #{self.update_count}")
        print(f"   World: {entity.world_address}")
        print(f"   Keys:  {entity.hashed_keys}")
        print(f"   Models: {len(entity.models)}")
        print(f"   Updated: {entity.updated_at}")
    
    def on_error(self, error):
        """Called when an error occurs"""
        print(f"\n❌ Subscription Error: {error}")


async def subscribe_to_entities(torii_url: str, world_address: str):
    """Subscribe to entity updates"""
    
    print(f"Connecting to Torii at {torii_url}...")
    
    # Create client
    client = await ToriiClient.new_with_config(torii_url, 4 * 1024 * 1024)
    
    print("✓ Connected successfully!")
    
    # Create callback handler
    callback = MyEntityCallback()
    
    print("\nSubscribing to entity updates...")
    print("Press Ctrl+C to stop\n")
    
    # Subscribe to entity updates
    # clause=None means subscribe to all entities
    # world_addresses=[world_address] filters by world
    subscription_id = await client.subscribe_entity_updates(
        clause=None,
        world_addresses=[world_address],
        callback=callback
    )
    
    print(f"✓ Subscribed with ID: {subscription_id}")
    print("Waiting for updates...\n")
    
    try:
        # Keep the script running to receive updates
        while True:
            await asyncio.sleep(1)
    except KeyboardInterrupt:
        print("\n\nUnsubscribing...")
        client.cancel_subscription(subscription_id)
        print(f"✓ Unsubscribed. Total updates received: {callback.update_count}")


async def main():
    torii_url = "http://localhost:8080"
    world_address = "0x0"
    
    if len(sys.argv) > 1:
        torii_url = sys.argv[1]
    if len(sys.argv) > 2:
        world_address = sys.argv[2]
    
    print("Dojo Python Example: Subscribe to Entities")
    print("==========================================\n")
    
    try:
        await subscribe_to_entities(torii_url, world_address)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        print(f"\nUsage: {sys.argv[0]} [torii_url] [world_address]")
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())

