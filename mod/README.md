# Skyrim mod integration (thin client contract)

This folder remains the Skyrim-side implementation placeholder. The runtime contract and expected behavior are now concrete.

## PoC goals

- On-demand trigger (hotkey/lesser power).
- Minimal event log payload.
- File bridge request/response handoff.
- Minimal UI rendering with clear status/error text.

## Runtime file-bridge contract (`v1`)

- Write request JSON: `Data/SKSE/Plugins/SkyrimLLMRuntime/requests/<request_id>.json`
- Read response JSON: `Data/SKSE/Plugins/SkyrimLLMRuntime/responses/<request_id>.json`

Contract fields are versioned and defined in runtime schemas:

- `runtime/contracts/recap-request-v1.schema.json`
- `runtime/contracts/recap-response-v1.schema.json`

## Error UX mapping expected by the mod

The runtime may return `ok=false` with error codes:

- `runtime_offline`: show "Runtime unavailable" message; no crash.
- `provider_error`: show "Provider failed, fallback used if available".
- `budget_exceeded`: show "Budget cap reached" with suggestion to retry later.
- `validation_failed`: show "Response invalid; safe fallback applied".
- `transport_error`: show "Bridge file error" and retry guidance.

## Integration notes

- No API keys or provider secrets in mod files/scripts.
- Spoiler-safe mode should be default for trigger UI.
- Keep the mod thin and provider-agnostic.
- Real Skyrim/VistulaRim validation is external to this repo and tracked via protocol docs.
