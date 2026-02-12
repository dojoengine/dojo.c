---
name: dojo-c-development
description: Contributor workflow for dojoengine/dojo.c. Use when implementing or reviewing changes to C bindings, UniFFI outputs, and multi-language binding build scripts in this repository.
---

# Dojo C Development

Use this skill to update `dojoengine/dojo.c` bindings and validate generated outputs consistently.

## Core Workflow

1. Confirm which target bindings are affected (C, C#, Go, Swift, Kotlin, C++, WASM).
2. Build the core library first:
   - `cargo build --release`
3. Run binding generation scripts relevant to the change:
   - `./scripts/build_c.sh`
   - `./scripts/build_csharp.sh`
   - `./scripts/build_go.sh`
   - `./scripts/build_swift.sh`
   - `./scripts/build_kotlin.sh`
   - `./scripts/build_cpp.sh`
   - `./scripts/build_wasm.sh`
4. For broad changes, use aggregate script:
   - `./scripts/build_all_bindings.sh`
5. Run quality checks:
   - `./scripts/rust_fmt.sh`
   - `./scripts/clippy.sh`

## PR Checklist

- Regenerate only bindings affected by the change (or explain full regen).
- Verify generated headers/artifacts are in sync with Rust API updates.
- Include exact build/check commands in PR notes.
