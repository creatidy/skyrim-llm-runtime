# Skyrim LLM Runtime (PoC)

This repository is a reference implementation plan and scaffold for **Skyrim LLM Runtime**: a Skyrim SE add-on (mod) plus a **local companion runtime** that demonstrates how to ship LLM features inside a game-like interactive loop with production-grade engineering disciplines.

The design keeps the **game layer thin and stable**, and puts complexity (LLM/TTS/STT, safety, caching, costs, observability, replay) into a local runtime that can evolve quickly.

## What is in scope for the PoC

The PoC proves the full loop is possible:

1. Skyrim can capture a minimal slice of game state + a small local event log on demand.
2. Skyrim can hand that payload to a local runtime (keys never in the mod).
3. The runtime can call OpenAI (LLM first), enforce constraints, cache results, and export replay bundles.
4. Skyrim can render the result and degrade gracefully when the runtime/provider is unavailable.

## Repository structure

- `docs/00-foundation/` - stable concept + product principles (includes the original concept document).
- `docs/10-plan/` - PoC definition, milestones, backlog, acceptance criteria.
- `docs/20-architecture/` - contracts and key architectural choices (transport, replay bundles, safety pipeline).
- `docs/30-runtime/` - runtime responsibilities, interfaces, and operational discipline.
- `docs/40-skyrim-mod/` - Skyrim-side integration notes (thin client posture, file bridge, minimal UI).
- `runtime/` - Rust workspace implementation (contracts, file transport, runtime CLI, safety/cache/replay).
- `mod/` - Skyrim mod integration placeholder and thin-client contract notes.

## Fundamentals to keep true

- On-demand by default (no always-on cloud calls).
- Local-first posture: provider keys and policy live outside the game process.
- Short structured outputs; strict validation; safe fallbacks.
- Debuggable by design: every output is traceable and replayable.

## Next action

Start at:
- `docs/10-plan/00-poc-definition.md`
- `docs/10-plan/01-poc-roadmap.md`
