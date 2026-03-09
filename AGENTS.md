# AGENTS.md

## Project overview

- This repo is a PoC for a Skyrim SE add-on plus a local companion runtime.
- `runtime/` is the main implementation surface today.
- `mod/` is a thin native client target and harness; it is not yet a finished Skyrim-deployable plugin.

## Sources of truth

- Start with `README.md` for the high-level map.
- For milestone status, use `docs/10-plan/01-poc-roadmap.md`, `docs/10-plan/02-backlog.md`, and `docs/10-plan/03-acceptance-checklist.md`.
- For architecture and contracts, use `docs/20-architecture/00-system-overview.md` and `docs/20-architecture/02-data-contracts.md`.
- For runtime work, use `runtime/README.md` and `docs/30-runtime/`.
- For Skyrim-side work, use `mod/README.md` and `docs/40-skyrim-mod/`.

## Key locations

- `docs/00-foundation/`: product intent and stable principles
- `docs/10-plan/`: PoC scope, roadmap, backlog, acceptance gates
- `docs/20-architecture/`: system boundaries, transport, contracts, ADRs
- `docs/30-runtime/`: runtime config, safety, provider, observability
- `docs/40-skyrim-mod/`: mod-side bridge, UI posture, validation protocol
- `runtime/`: Rust workspace for the local runtime
- `mod/`: C++ thin-client core, harness, and SKSE stub

## Code style guidelines

- Follow existing local style in each area instead of inventing a new one.
- Keep the Skyrim side thin; put complexity in `runtime/`.
- Prefer small, explicit interfaces and versioned contracts over implicit coupling.
- When editing contracts, update schemas and aligned code together.
- Do not mix provider logic into `mod/`.

## Working agreements

- Prefer changing `runtime/` unless the task truly belongs on the Skyrim side.
- When changing request/response shapes, keep code aligned with:
  - `runtime/contracts/recap-request-v1.schema.json`
  - `runtime/contracts/recap-response-v1.schema.json`
- If docs and code disagree, verify the code path and then update the docs.

## Setup commands

Runtime dev flow:

```bash
cd runtime
cargo run -p runtime-cli -- init-config --out config.dev.json
cargo run -p runtime-cli -- serve --transport file --config config.dev.json
```

In another shell:

```bash
cd runtime
cargo run -p runtime-cli -- simulate --config config.dev.json --spoiler-mode safe
```

Mod harness build:

```bash
cmake -S mod -B mod/build
cmake --build mod/build
./mod/build/skyrim_llm_mod_harness
```

## Build and test commands

Runtime checks:

```bash
cargo check --manifest-path runtime/Cargo.toml
cargo test --manifest-path runtime/Cargo.toml
```

## Testing instructions

- Run relevant tests for the area you changed.
- For runtime changes, at minimum run `cargo test --manifest-path runtime/Cargo.toml`.
- If contract behavior changes, verify simulator flow and replay output.
- If mod-side bridge behavior changes, validate request/response file paths and error mapping.
- If you list checks here, assume agents should run them before finishing.

## Security considerations

- `OPENAI_API_KEY` belongs only in the runtime environment.
- Keep redaction enabled by default unless the task explicitly requires otherwise.
- Do not add secrets, machine-specific paths, or private modpack assets to the repo.
- Preserve on-demand behavior; do not introduce always-on background provider calls.
- Keep outputs structured, validated, and safe by default.

## Documentation updates

- Keep `AGENTS.md` short and practical.
- Put durable project knowledge in `docs/` or package-specific READMEs, not here.
- When adding new major docs, link them from `README.md` or the relevant docs index.
