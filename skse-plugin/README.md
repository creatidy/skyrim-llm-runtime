# SKSE Plugin Host Scaffold

This folder is the thin Windows/SKSE host layer for the Skyrim-side mod.

It exists to solve one problem cleanly:

- keep the real mod logic in `mod/`
- keep the platform-specific Skyrim/SKSE glue in `skse-plugin/`

## What lives here

- host-side wrapper around `mod/include/skyrim_llm/plugin_api.hpp`
- callback plumbing for:
  - snapshot capture
  - status-line UI
  - message UI
  - recap hotkey dispatch
- a file layout ready for a future real SKSE/CommonLibSSE-NG shell

## What does not live here

- file bridge logic
- recap request creation
- response parsing
- runtime error mapping

That logic stays in `mod/`.

## Folder layout

- `include/skyrim_llm_skse_host/game_api.hpp`
  - abstract Skyrim-side snapshot source
- `include/skyrim_llm_skse_host/location_capture.hpp`
  - concrete TODO surface for player location capture
- `include/skyrim_llm_skse_host/game_time_capture.hpp`
  - concrete TODO surface for in-game time capture
- `include/skyrim_llm_skse_host/ui_api.hpp`
  - abstract Skyrim-side UI sink
- `include/skyrim_llm_skse_host/notification_ui.hpp`
  - concrete TODO surface for in-game notifications/message UI
- `include/skyrim_llm_skse_host/host_context.hpp`
  - owns the `plugin_api.hpp` handle and callback wiring
- `include/skyrim_llm_skse_host/hotkey_binding.hpp`
  - concrete TODO surface for recap hotkey registration
- `include/skyrim_llm_skse_host/skyrim_game_api.hpp`
  - concrete placeholder `GameApi` implementation
- `include/skyrim_llm_skse_host/skyrim_ui_api.hpp`
  - concrete placeholder `UiApi` implementation
- `include/skyrim_llm_skse_host/plugin_shell.hpp`
  - single object that owns the host wiring for a future real plugin entrypoint
- `include/skyrim_llm_skse_host/workflow.hpp`
  - small host-side helper workflow functions
- `src/host_context.cpp`
  - creates/destroys the `plugin_api` handle and forwards callbacks
- `src/hotkey.cpp`
  - recap hotkey dispatch helper
- `src/hotkey_binding.cpp`
  - CommonLib-style `RE::InputEvent` sink scaffold with clean unregister logic and a public `FeedKeyEvent(scan_code, pressed)` entrypoint
- `src/location_capture.cpp`
  - TODO location capture placeholder
- `src/game_time_capture.cpp`
  - TODO game-time capture placeholder
- `src/snapshot.cpp`
  - optional initial event-log seeding helper
- `src/notification_ui.cpp`
  - UI scaffold with separate HUD-notification and message-box presentation paths
- `src/ui.cpp`
  - host-ready UI helper
- `src/plugin_main.cpp`
  - plugin entrypoint scaffold that owns `SkyrimPluginShell`
- `src/plugin_shell.cpp`
  - concrete owner object for host init and hotkey dispatch
- `src/skyrim_game_api.cpp`
  - placeholder snapshot implementation until real Skyrim bindings exist
- `src/skyrim_ui_api.cpp`
  - placeholder UI implementation until real Skyrim bindings exist

## Current state

This folder is an in-repo scaffold, not yet a final Skyrim-loadable DLL.

What is ready:

- callback wiring to `mod/include/skyrim_llm/plugin_api.hpp`
- host-side structure
- plugin entrypoint scaffold around `SkyrimPluginShell`
- hotkey binding scaffold with CommonLib-style `RE::InputEvent` sink structure
- single hotkey dispatch path through `FeedKeyEvent(...)`
- UI scaffold with separate HUD-notification and message-box paths
- buildable portable scaffold target

What is still missing:

- real hotkey registration
- real Skyrim data extraction
- real in-game notifications/message UI
- final SKSE/CommonLibSSE-NG load-path confirmation in the real Windows build
- Windows DLL build + deployment into the MO2 mod folder

## Concrete Phase 2 TODO files

If you are wiring Skyrim for the first time, these are the files to finish and confirm in the real Windows build:

- `src/location_capture.cpp`
- `src/game_time_capture.cpp`
- `src/hotkey_binding.cpp`
- `src/notification_ui.cpp`

## Intended usage

The future Windows-side plugin shell should:

1. implement concrete `GameApi`
2. implement concrete `UiApi`
3. create `SkyrimPluginShell`
4. call `Initialize()`
5. dispatch the recap hotkey through `OnRecapHotkeyPressed()`

## Build note

`skse-plugin/CMakeLists.txt` builds only the host scaffold library.

It does not attempt to build a real Skyrim plugin in this Linux container.
The final DLL still has to be built in a Windows environment with SKSE/CommonLibSSE-NG dependencies.
