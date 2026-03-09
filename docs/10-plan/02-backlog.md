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
  - Thin integration shell, callback-facing API, and SKSE host scaffold are implemented in repo.
  - `hotkey_binding.cpp` now contains a CommonLib-style `RE::InputEvent` sink scaffold, clean unregister logic, and a public event-feed entrypoint.
  - Real in-game hotkey registration is still missing.
- `[x]` Add minimal event log collector.
- `[~]` Add file-bridge write/poll/render loop in real mod.
  - Native controller + bridge client + integration/UI abstractions are implemented.
  - `plugin_main.cpp` and `SkyrimPluginShell` scaffolds are implemented in repo.
  - `notification_ui.cpp` now has separate HUD-notification and message-box scaffolds.
  - Actual Skyrim snapshot capture and in-game UI binding are still missing.
- `[x]` Add thin Skyrim integration layer around the native controller:
  - `[x]` `skyrim_integration` orchestration layer
  - `[x]` `plugin_api` callback-facing ABI for a future SKSE shell
- `[x]` Add error UX mapping in game/native client:
  - `[x]` runtime offline
  - `[x]` provider error
  - `[x]` budget exceeded
  - `[x]` validation failed fallback

## Real integration (Iteration 3 gate)

- `[x]` Add real Skyrim validation protocol doc.
- `[x]` Add phased roundtrip tutorial for real Skyrim bring-up.
- `[x]` Add Phase 2 wiring/build/deploy guide for the Skyrim-side shell.
- `[x]` Add issue template for external modpack findings.
- `[ ]` Execute first VistulaRim smoke pass and capture findings.
- `[ ]` Convert integration findings into stabilization tasks.

## Next-up actionable tasks

1. Build the in-repo `skse-plugin/` target on Windows with `CommonLibSSE` available and validate the DLL output/deploy path.
2. Implement Phase 2 callbacks: snapshot capture, hotkey registration, and in-game notification/message presentation.
3. Perform first external real-Skyrim validation pass and log outcomes.
4. Convert integration findings into stabilization tasks.
5. Add CI/lint/format automation for runtime and mod harness builds.
