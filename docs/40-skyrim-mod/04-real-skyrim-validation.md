# Real Skyrim validation protocol (VistulaRim external environment)

This protocol defines the first mandatory real-Skyrim validation pass after simulator P1 E2E is green.

## Scope and posture

- Validation target: Skyrim AE + Creation Club + external VistulaRim modpack.
- This repository stays generic; no private modpack assets are committed.
- Reuse existing mods if it improves stability, speed, and failure reduction.

## Preconditions

- Simulator-based P1 flow passes: request -> response -> replay bundle.
- Runtime file bridge and fallback behavior verified in repo tests.
- Redaction enabled by default in runtime configuration.

## Smoke checklist

1. Trigger recap from in-game UI/hotkey.
2. Confirm request file appears in bridge requests folder.
3. Confirm runtime generates response file and replay bundle.
4. Confirm in-game recap render succeeds.
5. Confirm offline runtime path shows clear user-facing error.
6. Confirm provider failure path degrades safely (fallback/cached).
7. Confirm no provider secrets appear in mod files/logs.

## Capture and artifact rules

- Attach redacted request/response file pairs.
- Attach replay bundle directory (redacted by default).
- Record exact runtime build id, prompt version, provider model.
- Record modpack/version and top compatibility notes (no private assets).

## Exit criteria for Iteration 3 gate

- At least one successful real roundtrip.
- Offline and provider-failure UX validated in real environment.
- Any incompatibilities converted into backlog items with reproduction steps.
