# Contributing (Draft)

## Principles

- Keep the Skyrim layer thin: it should collect inputs, trigger requests, render UI, and optionally play audio.
- Put complexity in the runtime: safety, policy, caching, budgets, replay, observability, provider adapters.
- Prefer deterministic behavior and reproducibility over “magic”.

## How to propose changes

- Open an issue describing the problem and the proposed change.
- If the change affects architecture, add an ADR under `docs/20-architecture/adr/`.

## Development setup

- Preferred: open repo in VS Code Dev Container (`.devcontainer/devcontainer.json`).
- Runtime environment guide: `docs/30-runtime/05-devcontainer.md`.
- Standard quality gate inside container:
  - `cd runtime && cargo test`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
