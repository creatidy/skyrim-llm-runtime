# Event log (PoC)

Goal: provide enough context for a recap without heavy coupling to gameplay systems.

## Suggested minimal events

- Player location / cell name changes
- Quest objective text (when available)
- Time stamps (in-game or real)
- Optional user “bookmark note”

## Constraints

- Keep max N entries (e.g., 30–80) and cap total characters.
- Prefer short “facts”, not narrative.
- Redact paths/user identifiers before sending to runtime when possible (runtime also redacts).
