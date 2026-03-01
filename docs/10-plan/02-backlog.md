# Backlog (starter)

This backlog is written as small, testable items. Expand as implementation begins.

## Runtime

- Implement config loader (Player/Developer modes).
- Implement redaction rules (paths/usernames).
- Implement OpenAI adapter:
  - request building
  - structured output validation
  - timeout/retry policy
- Implement caching:
  - stable hash function
  - cache metadata (hit/miss, age)
- Implement replay bundles:
  - bundle folder layout
  - export toggle
  - local replay command
- Implement observability:
  - structured logs
  - minimal metrics output (JSON or /metrics)

## Skyrim mod

- Add on-demand trigger (hotkey or lesser power).
- Minimal event log:
  - timestamp
  - cell/location name
  - quest objective text when available
  - optional user “bookmark note”
- File bridge:
  - write request file
  - poll response file
  - display result
- Error UX:
  - runtime offline
  - provider error
  - budget exceeded
  - validation failed -> fallback recap

## Tooling

- Dev scripts:
  - run runtime in dev mode
  - replay a bundle
  - format/lint/ci stubs
