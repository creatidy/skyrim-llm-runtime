# Observability (starter)

## Metrics to track (minimum)

- request count by feature
- latency (p50/p95)
- provider errors and retries
- cache hit rate
- token usage estimates
- refusal / safety fallback counts

## Outputs

PoC options:
- `metrics.json` periodically updated
- `/health` and `/metrics` endpoints (later)

## Debuggability rule

Every response must be traceable to:
- request id
- runtime build id
- prompt/config version
- provider model identity
