# Runtime (PoC implementation)

This folder now contains an executable Rust workspace for the Skyrim LLM Runtime PoC.

## Workspace layout

- `Cargo.toml` - workspace definition.
- `contracts/` - JSON schemas for file-bridge request/response contracts (`v1`).
- `examples/` - sample request payloads.
- `bin/runtime-cli/` - CLI entrypoint (`serve`, `simulate`, `replay`, `init-config`).
- `crates/runtime-core/` - contracts, config, service orchestration, safety, cache, replay, observability.
- `crates/provider-openai/` - OpenAI provider adapter (mock/offline path with request building + structured parsing).
- `crates/transport-file/` - file bridge transport implementation.
- `bridge/` - local request/response folders for simulator-driven integration.
- `replay-bundles/` - deterministic replay artifacts.

## Contracts are source of truth

For the recap file bridge, these schemas define the public runtime interface and are versioned:

- `contracts/recap-request-v1.schema.json`
- `contracts/recap-response-v1.schema.json`

Rust types in `runtime-core` must remain aligned with these schema files.

## Quick start

1. Initialize a dev config (optional; defaults are built in):

```bash
cd runtime
cargo run -p runtime-cli -- init-config --out config.dev.json
```

You can also copy/edit `config.dev.example.json`.

2. Run runtime server loop (file transport):

```bash
cargo run -p runtime-cli -- serve --transport file --config config.dev.json
```

3. In another shell, run simulator request:

```bash
cargo run -p runtime-cli -- simulate --config config.dev.json --spoiler-mode safe
```

4. Replay a captured bundle:

```bash
cargo run -p runtime-cli -- replay --bundle replay-bundles/<request_id>
```

## CLI surface

- `serve --transport file [--config <path>] [--once]`
- `simulate [--config <path>] [--spoiler-mode safe|full]`
- `replay --bundle <path>`
- `init-config [--out <path>]`

## Testing

Run all workspace tests:

```bash
cargo test
```

If the local environment has no Rust toolchain, install `cargo` first, then rerun tests.

## Devcontainer

From repo root, open this project in a Dev Container to get a preconfigured Rust environment.

- Config: `.devcontainer/devcontainer.json`
- Bootstrap script: `.devcontainer/post-create.sh`
- Contributor guide: `docs/30-runtime/05-devcontainer.md`
