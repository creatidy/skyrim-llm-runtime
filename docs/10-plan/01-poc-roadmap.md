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

Status: **In progress (simulator E2E implemented)**

Delivered in repo:
- File-bridge transport implementation (`crates/transport-file`).
- OpenAI adapter (PoC offline/mock path) with request building, retry envelope, and structured parsing.
- Safety pipeline: redaction, output constraints, spoiler-safe behavior, fail-closed fallback.
- Caching: stable hash keying + metadata via file cache.
- Replay bundle export with deterministic layout.
- Simulator-driven E2E integration test and workflow.

Remaining for full P1 closure:
- Real OpenAI HTTP adapter path (optional for offline development, required for live provider validation).
- Skyrim-side real mod trigger/render integration in external environment.

Exit criteria:
- Hotkey -> recap displayed in game. **Pending external integration run**.
- No API key stored anywhere in the mod. **Met by architecture and docs**.
- Replay bundle generated and replayable without launching Skyrim. **Met in simulator flow**.

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
