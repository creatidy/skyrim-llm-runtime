# Devcontainer workflow (runtime contributors)

This project includes a root `.devcontainer` setup for reproducible Rust tooling.

## Why use it

- Same Rust toolchain and VS Code extensions for all contributors.
- No host machine Rust setup required.
- Faster onboarding for people new to Rust.

## Prerequisites

- Docker Desktop or Docker Engine.
- VS Code with `Dev Containers` extension.

## Start the container

1. Open repository root in VS Code.
2. Run `Dev Containers: Reopen in Container`.
3. Wait for `post-create` bootstrap to finish.

Bootstrap script:

- installs `rustfmt` and `clippy`
- runs `cargo fetch` in `runtime/`
- runs `cargo check` in `runtime/`

## Daily commands (inside container)

```bash
cd runtime
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

If `cargo fmt --check` fails, run `cargo fmt --all` and rerun the check.

## Simulator test loop (inside container)

Terminal 1:

```bash
cd runtime
cargo run -p runtime-cli -- init-config --out config.dev.json
cargo run -p runtime-cli -- serve --transport file --config config.dev.json
```

Terminal 2:

```bash
cd runtime
cargo run -p runtime-cli -- simulate --config config.dev.json --spoiler-mode safe
```

## Notes

- Bridge files are under `runtime/bridge/`.
- Replay bundles are under `runtime/replay-bundles/`.
- Metrics/logs are `runtime/metrics.json` and `runtime/runtime.log`.
- Codex state is persisted via Docker volume at `/home/vscode/.codex`, so Codex auth/config/skills survive container rebuilds.

## Scenario checks

- Runtime offline: run `simulate` without `serve`; expect timeout.
- Cache behavior: run `simulate` twice with `serve` running; second request should log `cache_hit=true`.
- Budget guard: set `budgets.max_tokens_per_call` very low and ensure cache miss (or disable cache), then run `serve` + `simulate`; expect `BudgetExceeded`.
- Fallback hooks: for manual request files in `runtime/bridge/requests/`, use `contract_version: "v1"` and include `[force_provider_error]` or `[force_invalid]` in event text to exercise fallback paths.
