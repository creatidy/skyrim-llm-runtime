# PoC roadmap

This roadmap is staged to prove feasibility early and then harden runtime discipline.

## Milestone P0 - Repo + contracts + runtime skeleton

Status: **Completed in repo (2026-03-02)**

Delivered:
- Versioned request/response contracts as JSON schema (`runtime/contracts/*v1.schema.json`).
- Runtime skeleton in Rust workspace with explicit interfaces:
  - `Transport`
  - `Provider`
  - `SafetyPipeline`
  - `CacheStore`
  - `ReplayStore`
- CLI surface:
  - `runtime serve --transport file --config <path>`
  - `runtime replay --bundle <path>`
  - plus `simulate` and `init-config` helper commands.
- Stub-to-real orchestration path capable of consuming request JSON and emitting response JSON.

Exit criteria:
- CLI/minimal service can ingest a Recap request and emit a Recap response. **Met**.

## Milestone P1 - End-to-end Recap (file bridge)

Status: **In progress (repo implementation complete, real Skyrim pass pending)**

Delivered in repo:
- File-bridge transport implementation (`crates/transport-file`).
- OpenAI adapter with offline/mock path plus live HTTP execution path.
- Safety pipeline: redaction, output constraints, spoiler-safe behavior, fail-closed fallback.
- Caching: stable hash keying + metadata via file cache.
- Replay bundle export with deterministic layout.
- Simulator-driven E2E integration test and workflow.
- In-repo Skyrim thin-client target under `mod/` with:
  - bounded event log collection
  - file-bridge request/response loop
  - hotkey-driven recap controller target
  - thin Skyrim integration layer (`skyrim_integration`) for snapshot/UI orchestration
  - thin plugin-facing callback API (`plugin_api`) for a future real SKSE shell
  - simple message-style UI mapping via native harness
- Env-only runtime secret posture (`OPENAI_API_KEY`).

Remaining for full P1 closure:
- First real Skyrim roundtrip in the target game environment.
  - Roundtrip tutorial and phased status live in `docs/40-skyrim-mod/05-first-real-roundtrip.md`.
  - Phase 1 (runtime path + real bridge folder preparation) is done in the current working setup.
  - Current focus is Phase 2: wiring the real Skyrim-side shell to the prepared integration layer under `mod/`.
- Final SKSE wiring from the prepared thin integration layer into actual in-game hotkey/UI hooks.
  - Implementation target and build/deploy guidance live in `docs/40-skyrim-mod/06-phase-2-skse-wiring.md`.

Exit criteria:
- Hotkey -> recap displayed in game. **Pending final in-game validation pass**.
- No API key stored anywhere in the mod. **Met in repo implementation**.
- Replay bundle generated and replayable without launching Skyrim. **Met in runtime flow**.

## Milestone P2 - Grounded in-game help (and optional narrator TTS)

Status: **Not started**

Planned deliverables:
- Help feature constrained to curated local documentation corpus.
- Strict grounding and citations to chunk ids/files.
- Optional narrator TTS (on-demand and cached).

## Milestone P3 - STT + evaluation harness + provider abstraction hardening

Status: **Not started**

Planned deliverables:
- Push-to-talk constrained intent grammar.
- Offline evaluation harness for regression/safety/grounding checks.
- Additional provider behind existing runtime provider interface.

## Integration gate policy

Switch from simulator-first to mandatory real Skyrim validation after simulator P1 E2E passes.

Reference:
- `docs/40-skyrim-mod/04-real-skyrim-validation.md`
