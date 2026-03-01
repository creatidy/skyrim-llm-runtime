# Skyrim mod integration (PoC)

## Responsibilities (thin client)

- Collect minimal inputs and local event log on demand.
- Serialize and write a request to a known folder.
- Poll and read the response.
- Render the output.
- Provide clear error messages.

## Non-negotiables

- No provider keys in the mod.
- No always-on background calling in early versions.
- Fail gracefully.
