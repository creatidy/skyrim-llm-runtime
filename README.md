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
- `runtime/` - Rust workspace implementation (contracts, file transport, runtime CLI, safety/cache/replay, live OpenAI path).
- `mod/` - Skyrim-side thin client implementation target and local native harness.

## Docs navigation logic

The `docs/` tree is organized by purpose, not by chronology alone.

- `00-foundation` answers "Why does this project exist, and what must stay true?"
- `10-plan` answers "What are we trying to prove in the PoC, what is done, and what is still open?"
- `20-architecture` answers "How is the system shaped, and which interfaces/contracts are authoritative?"
- `30-runtime` answers "How does the local runtime behave and how is it operated?"
- `40-skyrim-mod` answers "What belongs on the Skyrim side, and how thin should that layer stay?"

The numbering is intentional:

- Lower numbers are more stable and conceptual.
- Higher numbers are closer to implementation details and integration work.
- Within each folder, `00-...` files are entry points and the later files go narrower or more specific.

That means the expected reading order is usually:

1. `docs/00-foundation/` to understand the product and constraints.
2. `docs/10-plan/` to understand current milestone status.
3. `docs/20-architecture/` to understand contracts and technical boundaries.
4. `docs/30-runtime/` or `docs/40-skyrim-mod/` depending on which side of the system you are working on.

## If you are new to the repo

Use this path:

1. `docs/10-plan/00-poc-definition.md`
2. `docs/10-plan/01-poc-roadmap.md`
3. `docs/10-plan/03-acceptance-checklist.md`
4. `docs/20-architecture/00-system-overview.md`
5. `runtime/README.md` or `mod/README.md`

In short: `docs/` explains intent and boundaries, while `runtime/` and `mod/` contain the actual implementation targets.

## Fundamentals to keep true

- On-demand by default (no always-on cloud calls).
- Local-first posture: provider keys and policy live outside the game process.
- Short structured outputs; strict validation; safe fallbacks.
- Debuggable by design: every output is traceable and replayable.

## Next action

Start at:
- `docs/10-plan/00-poc-definition.md`
- `docs/10-plan/01-poc-roadmap.md`

## Development environment

This repo includes a root `.devcontainer` for consistent Rust setup and testing.

- Setup and usage: `docs/30-runtime/05-devcontainer.md`
