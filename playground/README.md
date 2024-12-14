# Playground

This is a playground for the Dojo client to be used in a browser, for development purposes.

The playground offers a simple interface to interact with the Dojo client, and to see the results of the operations.

The rule of thumb is that the playground should be as simple as possible, to focus the reader on the Dojo client and its features.

## Setup Dojo

To quickly test the Torii client, you need a dojo project. The easiest way to play with entities with different keys and models, is using the `examples/simple` project.

```bash
# Start a Katana.
katana --dev

# Build and migrate the project.
cd dojo/examples/simple
sozo build
sozo migrate
```
```bash
# Then you can execute systems where entities ids are exposed.
sozo execute c1 system_1 -c 1,1
# Use the system_5 to have an other model with 2 keys.
sozo execute c1 system_5 -c 1,1,2
```
```bash
# Run Torii to index the world. If you are working on torii (most probably), use
# cargo run:
cargo run --bin torii -r -- --world 0x064613f376f05242dfcc9fe360fa2ce1fdd6b00b1ce73dae2ea649ea118fd9be --http.cors_origins "*"
```

## Setup Playground

If you are working on adapting dojo.c to a new version of Torii, you might need to update the dojo dependencies in the `Cargo.toml` file. Change the `rev` is you've pushed to the dojo repository, or the `git`/`path` at your needs.

```toml
[dependencies]
dojo-world = { git = "https://github.com/dojoengine/dojo", rev = "180c6d2" }
dojo-types = { git = "https://github.com/dojoengine/dojo", rev = "180c6d2" }
torii-client = { git = "https://github.com/dojoengine/dojo", rev = "180c6d2" }
torii-grpc = { git = "https://github.com/dojoengine/dojo", features = [
    "client",
], rev = "180c6d2" }
torii-relay = { git = "https://github.com/dojoengine/dojo", rev = "180c6d2" }
```

```bash
# Ensure you have bunx installed:
https://bun.sh/docs/cli/bunx
```

```bash
# Build the wasm module for the browser.
bash scripts/build_wasm_web.sh
```

```bash
# Run a local server to serve the playground.
# Run this command at the root of the repository.
python3 -m http.server 8888
```

Open the browser at `http://localhost:8888/playground/entities.html`.

## Notes

Some browsers are not disabled cache by default for localhost, when you rebuild the playground, you might need to clear the cache / force the browser to reload the page.
