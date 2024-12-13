# Playground

This is a playground for the Dojo client to be used in a browser, for development purposes.

The playground offers a simple interface to interact with the Dojo client, and to see the results of the operations.

The rule of thumb is that the playground should be as simple as possible, to focus the reader on the Dojo client and its features.

## Setup

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
