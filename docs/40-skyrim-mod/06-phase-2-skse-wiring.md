# Phase 2 - Skyrim/SKSE Wiring

This document describes the actual current state of `mod/` and what still needs to be wired to finish Phase 2 of the first real Skyrim roundtrip.

## What is already implemented in repo

The repo already contains the reusable Skyrim-side native logic.

Core pieces:

- `mod/src/bridge_client.cpp`
  - writes request JSON
  - polls for response JSON
- `mod/src/controller.cpp`
  - creates recap requests
  - applies timeout handling
  - maps runtime errors to user-facing text
- `mod/src/skse_plugin_stub.cpp`
  - forwards hotkey/event calls into the controller
- `mod/src/skyrim_integration.cpp`
  - owns the thin integration flow:
    - ask for a `GameSnapshot`
    - show status/recap/error via a notification sink
    - trigger the recap request
- `mod/src/plugin_api.cpp`
  - exposes a thin callback-facing C API for a future real plugin shell
- `mod/src/harness_main.cpp`
  - local non-Skyrim harness for stepping the same flow

Key headers:

- `mod/include/skyrim_llm/controller.hpp`
- `mod/include/skyrim_llm/skse_plugin_stub.hpp`
- `mod/include/skyrim_llm/skyrim_integration.hpp`
- `mod/include/skyrim_llm/plugin_api.hpp`

The repo now also contains an in-repo host scaffold:

- `skse-plugin/`
  - thin host-side wrapper around `plugin_api.hpp`
  - callback-oriented `HostContext`
  - placeholder workflow files for hotkey, snapshot seeding, and UI

## What is still missing

The repo still does **not** contain the final Skyrim-loadable shell.

Missing work:

- real SKSE plugin entrypoint and load/init
- real hotkey registration
- real Skyrim snapshot capture
- real in-game UI binding
- final Windows plugin build/deployment flow using the `skse-plugin/` scaffold as the host layer

That is the work required to finish Phase 2.

## Intended architecture

The final shell should stay thin and call the callback API in `plugin_api.hpp`.

Responsibility split:

- real plugin shell:
  - register hotkey
  - collect snapshot data from Skyrim
  - map UI callbacks to Skyrim notifications/message boxes
  - call the C API through the host scaffold in `skse-plugin/`
- existing repo code:
  - file bridge
  - recap request creation
  - timeout handling
  - response parsing
  - error mapping

## Build and editor posture

You can stay in VS Code as the editor.

Important distinction:

- editor: VS Code is fine
- plugin target: must still be built as a Windows Skyrim/SKSE plugin

Practical requirement:

- build the Skyrim plugin from Windows, not from the Linux dev container

Typical prerequisites for that Windows-side build:

- Visual Studio C++ build tools installed on Windows
- CMake
- `vcpkg`
- a real SKSE/CommonLibSSE-NG plugin shell project
- VS Code C++ and CMake extensions

Recommended reference shell projects:

- CommonLibSSE-NG: <https://github.com/CharmedBaryon/CommonLibSSE-NG>
- SKSE template hello world: <https://github.com/SkyrimScripting/SKSE_Template_HelloWorld>

These references support opening the project in VS Code even though the actual compiler toolchain is Windows/MSVC.

## Deployment target

For the current VistulaRim setup, the intended MO2 mod path is:

```text
E:\Modding\VistulaRim\MO2\mods\SkyrimLLMRuntime
```

The plugin should deploy into:

```text
E:\Modding\VistulaRim\MO2\mods\SkyrimLLMRuntime\SKSE\Plugins\
```

The bridge folders are:

```text
E:\Modding\VistulaRim\MO2\mods\SkyrimLLMRuntime\SKSE\Plugins\SkyrimLLMRuntime\requests
E:\Modding\VistulaRim\MO2\mods\SkyrimLLMRuntime\SKSE\Plugins\SkyrimLLMRuntime\responses
```

## The callback contract you need to implement

`plugin_api.hpp` expects four things from the real Skyrim shell:

1. Create/destroy lifecycle.
2. Snapshot callback.
3. Status line callback.
4. Message callback.

### 1. Create the integration handle

The real shell should call `SkyrimLlm_Create(...)` during plugin initialization and pass:

- plugin title
- request folder path
- response folder path
- timeout
- callback function pointers
- plugin userdata/context pointer

### 2. Snapshot callback

The shell must implement `SkyrimLlmGetSnapshotFn`.

Minimum required snapshot fields:

- `player_location`
- optional `game_time`

Good enough for first smoke pass:

- current location/cell name
- a simple game-time string, or empty if not yet available

### 3. Status line callback

The shell must implement `SkyrimLlmShowStatusLineFn`.

Use this for:

- "Generating recap..."
- "Runtime unavailable"
- other short single-line errors

### 4. Message callback

The shell must implement `SkyrimLlmShowMessageFn`.

Use this for:

- recap summary
- numbered next steps
- simple cached/fresh note

## Hotkey wiring plan

For the first real pass, keep the hotkey implementation minimal.

Recommended plan:

1. Pick one default hotkey.
2. Register it once during plugin initialization.
3. On hotkey press:
   - call the snapshot callback
   - call `SkyrimLlm_TriggerHotkeyRecap(...)`

Optional but useful:

- call `SkyrimLlm_RecordLocationChange(...)` when location changes
- call `SkyrimLlm_RecordQuestObjective(...)` when objective text changes
- call `SkyrimLlm_RecordNote(...)` from a debug command or temporary test hook

## Build flow

Target flow for the real shell project:

1. Open the Windows-side plugin project in VS Code.
2. Configure CMake for the Windows toolchain.
3. Link or include this repo's `mod/include/`, `mod/src/`, and `skse-plugin/` host scaffold.
4. Build the plugin DLL.
5. Copy the DLL and optional PDB into:

```text
E:\Modding\VistulaRim\MO2\mods\SkyrimLLMRuntime\SKSE\Plugins\
```

6. Launch Skyrim through SKSE with the VistulaRim profile active.

## First-pass deployment checklist

1. Runtime is already running and watching the MO2 bridge folders.
2. Plugin DLL is present in `SKSE/Plugins/`.
3. `SkyrimLLMRuntime/requests/` and `responses/` folders exist.
4. Skyrim loads with the plugin enabled.
5. Hotkey press creates a request file.
6. Runtime writes a response file.
7. UI callback displays the recap.

## What "done" looks like for Phase 2

Phase 2 is done when all of these are true:

- plugin loads in the target Skyrim environment
- hotkey is registered and fires
- snapshot capture returns real game data
- request file is written to the real bridge folder
- response file is received
- recap or error text is shown in game

## Immediate next tasks for the user

1. Use `skse-plugin/` as the in-repo host layer.
2. Add the real Windows/SKSE entrypoint and concrete Skyrim bindings on top of it.
3. Implement the concrete `GameApi` and `UiApi` classes for Skyrim.
4. Build and deploy the DLL into the MO2 mod folder.
5. Run the first real roundtrip smoke pass.
