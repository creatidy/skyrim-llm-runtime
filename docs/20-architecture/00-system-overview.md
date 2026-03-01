# System overview

## Layers

1. Skyrim mod layer (thin client)
- Collects minimal inputs and event log (on demand).
- Sends a request to local runtime.
- Renders results and optional audio.

2. Local runtime (product layer)
- Stores credentials and policy.
- Orchestrates AI calls with caching, retries, timeouts.
- Enforces safety and privacy.
- Validates structured outputs and hard limits.
- Produces replay bundles and supports local replay.
- Exposes diagnostics and metrics.

3. Provider layer (OpenAI first)
- LLM for structured text generation.
- TTS/STT later.

## Design rule

The mod should not care which provider is used. Provider churn must not require Skyrim-side changes.
