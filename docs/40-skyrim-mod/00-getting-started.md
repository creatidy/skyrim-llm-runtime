# Skyrim mod integration (PoC)

## Responsibilities (thin client)

- Collect minimal inputs and local event log on demand.
- Serialize and write a request to a known folder.
- Poll and read the response.
- Render the output.
- Provide clear error messages.

## Repo layout

- `mod/` now contains the thin-client native core, a thin integration layer, a callback-facing plugin API, and a local harness.
- `runtime/` remains the provider/runtime side of the bridge.
- Real Skyrim hook-up should stay thin and call into the `mod/` controller layer.

## Current milestone posture

- Phase 1 of the first real roundtrip is done in the current working setup:
  - runtime is serving the real bridge path
  - bridge folders are known
  - runtime-side validation is complete
- Current focus is Phase 2:
  - connect a real Skyrim/SKSE shell to the prepared integration layer in `mod/`
  - capture a real snapshot from the running game
  - register a hotkey
  - render recap/error text in game

## Read this next

- `docs/40-skyrim-mod/05-first-real-roundtrip.md`
- `docs/40-skyrim-mod/06-phase-2-skse-wiring.md`

## If you are using the devcontainer

If the repo is opened in a container-only workspace, the Windows host will not see the container filesystem directly.

Before doing the real Windows SKSE/CommonLibSSE build, export a host-visible build tree:

```bash
bash .devcontainer/export-windows-build.sh
```

That stages the Windows build inputs in:

```text
E:\Modding\VistulaRim\skyrim-llm-runtime-build
```

Then build from:

```text
E:\Modding\VistulaRim\skyrim-llm-runtime-build\skse-plugin
```

## Non-negotiables

- No provider keys in the mod.
- No always-on background calling in early versions.
- Fail gracefully.
