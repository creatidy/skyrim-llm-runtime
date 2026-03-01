# PoC acceptance checklist

Status markers:
- `[x]` met in repo
- `[~]` partially met / simulator-only
- `[ ]` not yet met

## Functional

- `[~]` Press hotkey -> recap appears in game, or clear error message.
  - Simulator-driven request/response loop is implemented.
  - Real Skyrim trigger/render is pending external integration pass.
- `[x]` Recap output is short and readable.
- `[x]` Spoiler-safe mode is default.

## Security and privacy

- `[x]` Provider keys never exist in Skyrim scripts or mod files.
- `[~]` Runtime runs on localhost/default local file bridge.
  - File bridge is implemented; local HTTP is out of scope for these iterations.
- `[x]` Request payload is minimal and redacted.
- `[x]` Replay bundles are redacted by default posture.

## Reliability

- `[~]` If runtime is offline -> understandable message contract is defined.
  - In-game UX wiring is pending Skyrim mod implementation.
- `[x]` If provider fails -> fallback behavior works (cached or safe template).
- `[x]` Validation failures fail closed.

## Debuggability

- `[x]` Every output includes:
  - request id
  - runtime build id
  - prompt/config version
  - provider model identity
- `[x]` Replay bundle can be replayed outside Skyrim.

## Cost control

- `[x]` On-demand only.
- `[x]` Caching works (identical request produces cache hit behavior).
- `[x]` Budget caps are enforced by runtime.

## External validation gate (Iteration 3)

- `[ ]` First real Skyrim roundtrip succeeds with clear degradation behavior.
- `[ ]` Simulator suite remains green after integration adjustments.
- `[ ]` Integration findings documented and converted into backlog items.
