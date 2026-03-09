# Backlog

Backlog items are small and testable. Status markers:
- `[x]` done in repo
- `[~]` in progress / partial
- `[ ]` not started

## Runtime

- `[x]` Implement config loader (Player/Developer modes).
- `[x]` Implement redaction rules (paths/usernames/machine names baseline).
- `[x]` Implement OpenAI adapter:
  - `[x]` request building
  - `[x]` structured output parsing + validation handoff
  - `[x]` timeout/retry envelope (offline/mock path)
  - `[x]` live HTTP call path for real provider execution
- `[x]` Implement caching:
  - `[x]` stable hash function
  - `[x]` cache metadata (hit/miss, age)
- `[x]` Implement replay bundles:
  - `[x]` bundle folder layout
  - `[x]` export toggle
  - `[x]` local replay command
- `[x]` Implement observability baseline:
  - `[x]` structured logs
  - `[x]` minimal metrics output (`metrics.json`)

## Contracts and interfaces

- `[x]` Add versioned recap schemas (`v1`).
- `[x]` Add aligned Rust request/response types.
- `[x]` Add runtime interfaces: Transport/Provider/Safety/Cache/Replay.
- `[x]` Add schema + roundtrip tests.

## Simulator and tooling

- `[x]` Add file bridge simulator workflow.
- `[x]` Add `runtime-cli` commands for serve/simulate/replay/init-config.
- `[~]` Add CI/lint/format stubs.

## Skyrim mod

- `[~]` Add on-demand trigger (hotkey or lesser power).
- `[x]` Add minimal event log collector.
- `[~]` Add file-bridge write/poll/render loop in real mod.
- `[x]` Add error UX mapping in game/native client:
  - `[x]` runtime offline
  - `[x]` provider error
  - `[x]` budget exceeded
  - `[x]` validation failed fallback

## Real integration (Iteration 3 gate)

- `[x]` Add real Skyrim validation protocol doc.
- `[x]` Add issue template for external modpack findings.
- `[ ]` Execute first VistulaRim smoke pass and capture findings.
- `[ ]` Convert integration findings into stabilization tasks.

## Next-up actionable tasks

1. Wire the SKSE stub to real in-game hotkey and message presentation.
2. Perform first external real-Skyrim validation pass and log outcomes.
3. Convert integration findings into stabilization tasks.
4. Add CI/lint/format automation for runtime and mod harness builds.
5. Start P2 prep: grounded help data pipeline and chunk citation contract draft.
