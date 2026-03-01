# Runtime configuration (starter)

Keep configuration explicit and user-controlled.

## Config sections

- mode: player | developer
- openai:
  - api_key (from env or secrets store)
  - model
  - timeout_ms
- budgets:
  - max_calls_per_hour
  - max_tokens_per_call
  - daily_spend_cap (estimate)
- caching:
  - enabled
  - ttl_seconds
- privacy:
  - redaction_enabled
  - retention_policy
- replay:
  - export_enabled
  - directory

## Key rule

Never require the user to put API keys into Skyrim mod files.
