# Skyrim mod integration (real implementation target)

`mod/` now holds the Skyrim-side thin client target for P1 while `runtime/` stays provider/runtime focused.

## What is implemented here

- Native thin-client core in C++20:
  - bounded event log
  - file-bridge request writer
  - response polling
  - recap/error mapping
- `skyrim_llm_mod_harness` executable for local stepping outside Skyrim
- `skyrim_llm_skse_stub` library that shows where SKSE hotkey and UI hooks attach

## P1 responsibilities

- On-demand hotkey trigger
- Minimal event log payload
- File bridge request/response handoff
- Simple notification/message-style recap display
- Clear offline/provider/budget/validation error mapping

## Bridge contract

- Write request JSON: `Data/SKSE/Plugins/SkyrimLLMRuntime/requests/<request_id>.json`
- Read response JSON: `Data/SKSE/Plugins/SkyrimLLMRuntime/responses/<request_id>.json`

Contract fields stay versioned in:

- `runtime/contracts/recap-request-v1.schema.json`
- `runtime/contracts/recap-response-v1.schema.json`

## Local harness

Build from repo root:

```bash
cmake -S mod -B mod/build
cmake --build mod/build
```

Run with the runtime already serving the file bridge:

```bash
./mod/build/skyrim_llm_mod_harness
```

The harness simulates the hotkey path and uses the same request/response folders as the runtime PoC bridge.

## SKSE-first posture

The runtime-facing logic lives in `skyrim_llm_mod_core`. A real SKSE plugin should stay thin:

- capture hotkey/input
- gather current location/objective data
- call `RecapController`
- present the returned recap/error in game

No provider keys or provider logic belong in `mod/`.
