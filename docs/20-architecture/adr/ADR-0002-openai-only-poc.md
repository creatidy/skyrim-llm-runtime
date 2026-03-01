# ADR-0002: OpenAI only for PoC

## Status
Accepted (PoC)

## Context
The PoC should validate feasibility with one provider first.

## Decision
Use OpenAI as the only provider in PoC.

## Consequences
- Faster implementation and clearer debugging.
- Provider abstraction still exists as an interface, but only one adapter is implemented initially.
