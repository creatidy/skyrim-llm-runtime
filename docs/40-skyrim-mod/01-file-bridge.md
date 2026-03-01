# File bridge (PoC)

## Folder convention (proposal)

Data/SKSE/Plugins/SkyrimLLMRuntime/
- requests/
- responses/
- logs/ (optional)

## Lifecycle

1. Skyrim writes `requests/<request_id>.json`
2. Runtime consumes it and writes `responses/<request_id>.json`
3. Skyrim polls for the response with a timeout
4. Skyrim displays recap or error message
5. Runtime optionally archives or deletes processed requests

## Debug tip

Because files are visible artifacts, users can attach request/response pairs to bug reports.
