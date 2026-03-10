# Skyrim mod integration (real implementation target)

`mod/` now holds the Skyrim-side thin client target for P1 while `runtime/` stays provider/runtime focused.

## What is implemented here

- Native thin-client core in C++20:
  - bounded event log
  - file-bridge request writer
  - response polling
  - recap/error mapping
- Thin Skyrim integration layer:
  - snapshot provider abstraction
  - notification/message abstraction
  - recap trigger orchestration
- Thin plugin-facing callback API:
  - create/destroy integration handle
  - record events
  - trigger hotkey recap
- `skyrim_llm_mod_harness` executable for local stepping outside Skyrim
- `skyrim_llm_skse_stub` library that forwards hotkey and event calls into the core controller

## P1 responsibilities

- On-demand hotkey trigger
- Minimal event log payload
- File bridge request/response handoff
- Simple notification/message-style recap display
- Clear offline/provider/budget/validation error mapping

## Bridge contract

- Write request JSON: `Data/SKSE/Plugins/SkyrimLLMRuntime/requests/<request_id>.json`
- Read response JSON: `Data/SKSE/Plugins/SkyrimLLMRuntime/responses/<request_id>.json`

For the real MO2-based integration pass, the more reliable setup is a shared absolute bridge directory outside MO2 virtualization, configured consistently for both the runtime and the Windows plugin build.

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

## Code map

- `src/bridge_client.cpp`
  - writes request files and polls for response files
- `src/controller.cpp`
  - owns recap request creation, timeout handling, and error mapping
- `src/skse_plugin_stub.cpp`
  - thin forwarding layer over `RecapController`
- `src/skyrim_integration.cpp`
  - thin orchestration layer for snapshot capture, notifications, and recap triggering
- `src/plugin_api.cpp`
  - C ABI intended for a real Skyrim/SKSE shell to call
- `src/harness_main.cpp`
  - local non-Skyrim stepping path that exercises the same bridge flow

Public headers:

- `include/skyrim_llm/controller.hpp`
- `include/skyrim_llm/skse_plugin_stub.hpp`
- `include/skyrim_llm/skyrim_integration.hpp`
- `include/skyrim_llm/plugin_api.hpp`

## SKSE-first posture

The runtime-facing logic lives in `skyrim_llm_mod_core` and `skyrim_llm_skyrim_integration`. A real SKSE plugin should stay thin:

- capture hotkey/input
- gather current location/objective data
- implement the callbacks required by `plugin_api.hpp`
- call the plugin API on hotkey/input events
- present the returned recap/error in game via notification/message UI

No provider keys or provider logic belong in `mod/`.

## Next docs

- `docs/40-skyrim-mod/05-first-real-roundtrip.md`
- `docs/40-skyrim-mod/06-phase-2-skse-wiring.md`
