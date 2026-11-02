# ADR-001: Pure-function VT parser (no side-effects)

**Status:** Accepted  
**Date:** 2024-01-01

## Context

Terminal emulators historically intermix parsing and rendering, making them
hard to test. We want deterministic unit tests with no real PTY.

## Decision

`VtParser::feed` is a pure state machine that only writes to `self.printable`.
All side-effects (cursor movement, colour changes) are deferred to a renderer
layer that reads parser output.

## Consequences

- Parser is trivially testable without a real PTY or display.
- Renderer and parser can evolve independently (separation of concerns).
- Slightly more code (two-layer architecture) versus a monolithic approach.
