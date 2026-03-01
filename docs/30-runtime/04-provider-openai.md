# OpenAI provider adapter (PoC)

## Responsibilities

- Build the request for the model.
- Enforce timeouts and retry policy.
- Parse and validate structured outputs (JSON schema).
- Return a `RecapResponse` with meta, budgets, and cache info.

## PoC behavior

- LLM only (text recap).
- Short, structured outputs only.
- Deterministic metadata and version stamping for replay.

## Future extensions

- TTS narrator output (optional, cached).
- STT push-to-talk intent capture (constrained).
