# PoC acceptance checklist

Status markers:
- `[x]` met in repo
- `[~]` partially met / simulator-only
- `[ ]` not yet met

## Functional

- `[~]` Press hotkey -> recap appears in game, or clear error message.
  - Runtime and in-repo mod-side bridge loop are implemented.
  - Runtime/bridge preparation for the real environment is complete.
  - Current work is Phase 2: real Skyrim-side hotkey/UI/snapshot wiring.
  - See:
    - `docs/40-skyrim-mod/05-first-real-roundtrip.md`
    - `docs/40-skyrim-mod/06-phase-2-skse-wiring.md`
- `[x]` Recap output is short and readable.
- `[x]` Spoiler-safe mode is default.

## Security and privacy

- `[x]` Provider keys never exist in Skyrim scripts or mod files.
- `[~]` Runtime runs on localhost/default local file bridge.
  - File bridge is implemented end to end in repo; real Skyrim environment validation is pending.
- `[x]` Request payload is minimal and redacted.
- `[x]` Replay bundles are redacted by default posture.

## Reliability

- `[~]` If runtime is offline -> understandable message contract is defined.
  - Runtime-side simulator timeout test and native client error mapping are implemented.
  - Real Skyrim message rendering still needs a final smoke pass.
  - Phase 2 wiring is the current dependency.
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
  - Bring-up plan: `docs/40-skyrim-mod/05-first-real-roundtrip.md`
- `[x]` Simulator suite remains green after integration adjustments.
- `[ ]` Integration findings documented and converted into backlog items.
