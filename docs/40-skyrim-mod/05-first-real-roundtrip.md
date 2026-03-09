# First Real Skyrim Roundtrip

This document turns the remaining P1 item "First real Skyrim roundtrip in the target game environment" into a concrete phased bring-up plan.

## Goal

Prove this exact loop in a real Skyrim session:

1. Press hotkey in game.
2. Skyrim writes `requests/<request_id>.json`.
3. Runtime reads it and writes `responses/<request_id>.json`.
4. Skyrim renders recap or error text.
5. Replay bundle is exported.

This closes the first half of the remaining P1 work. The second half is the final SKSE wiring documented in `docs/40-skyrim-mod/06-phase-2-skse-wiring.md`.

## Current status

- Phase 1 is done in the current working setup.
- Current focus is Phase 2.

## Phase 1 - Runtime and bridge preparation

Status: **Done**

Done means:

- Runtime is already running against the real bridge folder.
- Bridge folders under the Skyrim/MO2 mod path are known and writable.
- Runtime config is pointed at the real `requests/` and `responses/` folders.
- Runtime-side tests and simulator flow already pass in repo.

This phase is complete. Do not spend time here unless paths change.

## Phase 2 - Real Skyrim-side wiring

Status: **Current**

Goal:

- Load a real Skyrim-side plugin shell.
- Register one recap hotkey.
- Capture a minimal `GameSnapshot`.
- Call the prepared integration layer in `mod/`.
- Show recap/error text in game.

Detailed implementation guidance:

- `docs/40-skyrim-mod/06-phase-2-skse-wiring.md`

Exit criteria for Phase 2:

- Hotkey press triggers the recap path.
- Request file appears in the real bridge folder.
- Response file is picked up and shown in game.

## Phase 3 - Real environment validation pass

Goal:

- Run the smoke checklist from `docs/40-skyrim-mod/04-real-skyrim-validation.md`.
- Capture artifacts and findings.

Tasks:

1. Run one successful recap roundtrip.
2. Validate offline runtime UX.
3. Validate provider-failure UX.
4. Save redacted request/response artifacts.
5. Save replay bundle artifacts.
6. Record exact build/config/model identifiers.

Exit criteria:

- At least one successful real roundtrip.
- Offline and provider-failure behavior validated.
- Findings documented.

## Phase 4 - Stabilization

Goal:

- Convert findings from the first real pass into backlog tasks and cleanup work.

Tasks:

1. Fix path assumptions, timing issues, or UI rough edges.
2. Tighten hotkey handling if needed.
3. Improve deployment ergonomics.
4. Update docs with confirmed working steps.

Exit criteria:

- Findings converted into explicit backlog items or resolved directly.

## Fast checklist

Use this when you return to the real environment:

1. Start the runtime.
2. Launch Skyrim through SKSE in the target modpack.
3. Load a save.
4. Press the recap hotkey.
5. Check `requests/` and `responses/`.
6. Confirm recap UI appears.
7. Repeat with runtime stopped.
8. Repeat with provider failure path.
