# Data contracts (starter)

This defines minimal, versioned contracts. Keep them stable and extend by versioning.

## RecapRequest (v1)

- request_id: string (uuid)
- feature: "recap"
- created_at_utc: string (ISO8601)
- spoiler_mode: "safe" | "full"
- game_context:
  - game_time: string (in-game date/time, if available)
  - player_location: string
  - event_log: array of EventEntry (max N)
- client:
  - client_kind: "skyrim-mod"
  - client_version: string
  - profile: "player" | "developer"

## EventEntry (v1)

- t: string (timestamp or in-game time)
- kind: string (e.g., "location", "quest", "note")
- text: string (already minimized)

## RecapResponse (v1)

- request_id: string
- ok: boolean
- recap:
  - summary: string
  - next_steps: array[string] (3-5)
  - spoiler_risk: "none" | "low" | "medium" | "high"
- meta:
  - runtime_build_id: string
  - prompt_version: string
  - provider:
    - name: string
    - model: string
  - cache:
    - hit: boolean
    - key: string (optional)
  - budgets:
    - tokens_in: number (optional)
    - tokens_out: number (optional)
- error:
  - code: string
  - message: string

## Versioning rule

Any breaking change increments the contract version and uses a new file name or version field.
