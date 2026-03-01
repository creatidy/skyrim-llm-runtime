# PoC roadmap

This roadmap is intentionally staged to prove feasibility early and then harden the runtime discipline.

## Milestone P0 - Repo + contracts + runtime skeleton

Deliverables:
- Repository scaffold and documentation (this repo).
- Versioned request/response contract for Recap.
- Replay bundle format definition.
- Runtime skeleton with “Provider adapter” and “Transport” interfaces.

Exit criteria:
- A CLI or minimal service can ingest a Recap request and emit a stubbed Recap response.

## Milestone P1 - End-to-end Recap (file bridge)

Deliverables (Skyrim side):
- Hotkey or simple UI trigger to generate Recap.
- Minimal event log collector (last N events).
- Writes `recap_request.json` to a known folder.
- Polls for `recap_response.json`.
- Renders recap text in the simplest UI (messagebox is fine).

Deliverables (runtime):
- File-bridge transport: watch for requests, write responses.
- OpenAI LLM call with hard constraints (structured output).
- Safety pipeline: redaction + validation + fallback.
- Caching keyed by stable hash.
- Replay bundle export (always on in Developer mode).

Exit criteria:
- Hotkey -> recap displayed in game.
- No API key stored anywhere in the mod.
- Replay bundle is generated and replayable without launching Skyrim.

## Milestone P2 - Grounded in-game help (and optional narrator TTS)

Deliverables:
- Help feature that answers only from a curated local documentation set.
- Strict grounding and citations to chunk ids / files.
- Optional narrator TTS for recap (on-demand, cached).

Exit criteria:
- Help answers never invent information outside the provided docs.
- TTS can be disabled, and all voice output is clearly optional.

## Milestone P3 - STT + evaluation harness + provider abstraction hardening

Deliverables:
- Push-to-talk voice commands mapped to a small, constrained intent set.
- Evaluation harness for safety, grounding, and regression tests.
- Add second provider (Ollama or other) behind the runtime interface.

Exit criteria:
- Voice control is constrained and safe.
- Offline evaluation can detect regressions.
- Provider swap does not require changes in Skyrim layer.
