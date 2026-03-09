# OpenAI provider adapter (PoC)

## Responsibilities

- Build the request for the model.
- Execute the live Responses API path when `mock_mode=false`.
- Enforce timeouts and retry policy.
- Parse structured outputs before runtime safety validation.
- Return provider metadata and token estimates for cache/replay/debug flows.

## PoC behavior

- LLM only (text recap).
- Short, structured outputs only.
- Deterministic metadata and version stamping for replay.
- `OPENAI_API_KEY` comes from the runtime environment only.
- `mock_mode=true` remains the default for offline development.

## Future extensions

- TTS narrator output (optional, cached).
- STT push-to-talk intent capture (constrained).
