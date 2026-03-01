# PoC acceptance checklist

## Functional

- Press hotkey -> recap appears in game, or shows a clear error message.
- Recap output is short and readable.
- Spoiler-safe mode is default.

## Security and privacy

- Provider keys never exist in Skyrim scripts or mod files.
- Runtime runs on localhost by default.
- Request payload is minimal and redacted.
- Replay bundles are redacted by default.

## Reliability

- If runtime is offline -> Skyrim shows understandable message and does not crash.
- If provider fails -> fallback behavior works (cached or safe template).
- Validation failures fail closed.

## Debuggability

- Every output includes:
  - request id
  - runtime build id
  - prompt/config version
  - provider model identity
- Replay bundle can be replayed outside Skyrim.

## Cost control

- On-demand only.
- Caching works (second identical request is a cache hit).
- Budget caps are enforced by runtime.
