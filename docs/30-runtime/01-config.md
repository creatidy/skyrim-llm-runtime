# Runtime configuration (starter)

Keep configuration explicit and user-controlled.

## Config sections

- mode: player | developer
- openai:
  - model
  - timeout_ms
  - max_retries
  - mock_mode
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

`OPENAI_API_KEY` must come from the runtime environment. Never require the user to put API keys into Skyrim mod files or runtime JSON config.
