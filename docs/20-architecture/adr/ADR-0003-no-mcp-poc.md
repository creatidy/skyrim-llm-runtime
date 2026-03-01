# ADR-0003: No MCP in PoC

## Status
Accepted (PoC)

## Context
MCP adds a standard tool/resource protocol. The PoC needs only one feature and one provider.

## Decision
Do not introduce MCP for PoC.

## Consequences
- Runtime stays smaller.
- Keep an internal “tools” interface so MCP can be adopted later if a tool ecosystem becomes a goal.
