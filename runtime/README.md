# Runtime (PoC implementation)

This folder now contains an executable Rust workspace for the Skyrim LLM Runtime PoC.

## Workspace layout

- `Cargo.toml` - workspace definition.
- `contracts/` - JSON schemas for file-bridge request/response contracts (`v1`).
- `examples/` - sample request payloads.
- `bin/runtime-cli/` - CLI entrypoint (`serve`, `simulate`, `replay`, `init-config`).
- `crates/runtime-core/` - contracts, config, service orchestration, safety, cache, replay, observability.
- `crates/provider-openai/` - OpenAI provider adapter (mock/offline path plus live Responses API execution).
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

For live OpenAI execution, keep `mock_mode` set to `false` and export `OPENAI_API_KEY` in the runtime shell. API keys are not stored in config JSON.

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
- `simulate [--config <path>] [--spoiler-mode safe|full] [--timeout-ms <ms>]`
- `replay --bundle <path>`
- `init-config [--out <path>]`

## Testing

Run all workspace tests:

```bash
cargo test
```

If the local environment has no Rust toolchain, install `cargo` first, then rerun tests.

## Verified contributor procedure

From `runtime/`:

1. Install/verify toolchain (once per machine):

```bash
rustup toolchain install stable
rustup default stable
rustup component add rustfmt clippy
```

2. Fast correctness pass:

```bash
cargo check
```

3. Full tests:

```bash
cargo test
```

4. Quality checks:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

If `cargo fmt --check` fails, run `cargo fmt --all` once and rerun the check.

5. Manual runtime loop (two terminals):

Terminal A:

```bash
cd runtime
cargo run -p runtime-cli -- init-config --out config.dev.json
cargo run -p runtime-cli -- serve --transport file --config config.dev.json
```

Terminal B:

```bash
cd runtime
cargo run -p runtime-cli -- simulate --config config.dev.json --spoiler-mode safe
```

Live mode:

```bash
cd runtime
export OPENAI_API_KEY=...
cargo run -p runtime-cli -- serve --transport file --config config.dev.json
```

6. Inspect artifacts:

- Replay bundles: `runtime/replay-bundles/`
- Bridge request/response files: `runtime/bridge/requests/`, `runtime/bridge/responses/`
- Metrics: `runtime/metrics.json`
- Logs: `runtime/runtime.log`

## Practical scenario tests

1. Runtime offline:
- Do not start `serve`.
- Run `simulate --timeout-ms 500`.
- Expect client timeout (`timeout waiting for response`).

2. Cache hit:
- Start `serve`.
- Run `simulate` twice.
- Expect `cache_hit=true` in server output for the second run.

3. Budget exceeded:
- Set `budgets.max_tokens_per_call` very low (for example `10`).
- Ensure this request path is not served from fresh cache (disable caching or use a cache-miss request).
- Run `serve` + `simulate`.
- Expect `ok=false` with `error_code=BudgetExceeded`.

4. Provider error fallback:
- Create a request file in `runtime/bridge/requests/` with event text containing `[force_provider_error]`.
- Use `contract_version: "v1"` in that manual request.
- Expect fallback recap response (or stale cache when available).

5. Validation-failure fallback:
- Create a request file with `[force_invalid]` in event text.
- Use `contract_version: "v1"` in that manual request.
- Expect fallback recap response.

## Devcontainer

From repo root, open this project in a Dev Container to get a preconfigured Rust environment.

- Config: `.devcontainer/devcontainer.json`
- Bootstrap script: `.devcontainer/post-create.sh`
- Contributor guide: `docs/30-runtime/05-devcontainer.md`
