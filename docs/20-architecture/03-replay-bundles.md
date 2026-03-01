# Replay bundles

## Goal

Make every output reproducible for debugging and regression.

## Bundle layout (v1)

replay-bundles/<request_id>/
- request.json
- response.json
- runtime_build_id.txt
- prompt_version.txt
- provider.json
- timestamps.json
- redaction_report.json (optional)
- notes.txt (optional)

## Rules

- Replay bundles are redacted by default.
- Developer mode exports bundles automatically.
- Player mode can export bundles on demand (opt-in).

## Replay runner

A runtime command that loads a request bundle and replays it with:
- same prompt version and config (if available)
- same provider/model (if available)
or an explicitly selected provider for comparison.
