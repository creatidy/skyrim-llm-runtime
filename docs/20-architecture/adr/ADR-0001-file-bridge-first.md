# ADR-0001: Use file bridge transport for PoC

## Status
Accepted (PoC)

## Context
We need a reliable bridge between Skyrim and a local runtime that works across Windows and Linux/Proton setups.

## Decision
Use a file-based request/response bridge for the PoC.

## Consequences
- Pros: reliability, debuggability, replay friendliness.
- Cons: polling and file lifecycle management required.
- Mitigation: define a runtime `Transport` interface so HTTP can be added later without changing feature logic.
