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
cargo test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

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
