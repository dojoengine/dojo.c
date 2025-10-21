# Dojo C Example

This example demonstrates how to use the Dojo C bindings to interact with a Dojo world.

## Prerequisites

1. Build the Dojo C library:
   ```bash
   cargo build --release -p dojo-c
   ```

2. Make sure you have a local Katana node running with a deployed Dojo world.

## Building

```bash
./build.sh
```

Or manually:
```bash
gcc -o main main.c \
    -I../../ \
    -L../../target/release \
    -ldojo_c \
    -Wl,-rpath,../../target/release
```

## Running

```bash
./main
```

## Features

This example demonstrates:
- Connecting to a Torii indexer
- Creating and connecting a controller account (Cartridge)
- Executing transactions through the controller
- Fetching entities from the world
- Subscribing to entity updates
- Using the Starknet account abstraction

