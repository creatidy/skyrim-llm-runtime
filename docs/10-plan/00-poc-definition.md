# PoC definition

## Primary objective

Prove the end-to-end loop is feasible and shippable:

1. Skyrim collects minimal game state + a small event log, on demand (hotkey/UI).
2. Skyrim passes a request to a local runtime (no provider keys in the mod).
3. Runtime calls OpenAI (LLM first), enforces safety + constraints, caches, and exports replay bundles.
4. Skyrim renders the result and degrades gracefully when runtime/provider is unavailable.

## PoC feature (player-facing)

**Last Session Recap and Next Steps**

- Short recap of recent progress.
- 3–5 suggested next actions.
- Spoiler-safe mode is default.
- Minimal UI is acceptable.

## Platform disciplines included in PoC (the “moat” starts now)

- Prompt/config/tool versioning stamped into every output.
- Deterministic replay bundles (request + config + output + metadata).
- Safety pipeline (input minimization/redaction + hard output constraints + fallback behavior).
- Observability basics (structured logs + minimal metrics).
- Cost controls (on-demand only + caching + budgets).

## Out of scope for PoC

- STT push-to-talk
- TTS narrator voice
- HTTP transport (optional later)
- Multi-provider support (OpenAI only in PoC)
- MCP integration
