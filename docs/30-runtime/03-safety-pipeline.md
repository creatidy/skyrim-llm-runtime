# Safety pipeline (starter)

Safety is a pipeline, not a single filter.

## Inputs

- Data minimization: send only what the feature needs.
- Redaction: file paths, usernames, machine names.
- Size limits: cap event log length and overall request size.

## Outputs

- Enforce a strict output schema (structured outputs).
- Hard length limits.
- Spoiler-safe rules for gameplay recap by default.
- Fail closed: if validation fails, return a safe fallback.

## Fallback behavior

- Prefer cached output if available.
- Otherwise return a short, safe template recap with clear messaging.
