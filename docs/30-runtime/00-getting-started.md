# Runtime getting started (implementation notes)

## Responsibilities

- Store and protect provider credentials.
- Orchestrate provider calls (LLM now; TTS/STT later).
- Enforce safety and privacy rules.
- Validate structured outputs and apply hard limits.
- Cache results and enforce budgets.
- Export replay bundles and support local replay.
- Emit structured logs and basic metrics.

## Modes

Player mode:
- minimal logs/metrics
- replay export off by default
- conservative budgets

Developer mode:
- dashboard/metrics enabled
- replay export enabled
- more detailed logs (still redacted by default)
- evaluation runner available
