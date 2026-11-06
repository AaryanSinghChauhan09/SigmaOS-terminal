# ADR-001: Terminal Design Decisions

**Date:** 2024-01-01  
**Status:** Accepted  
**Component:** SigmaOS-terminal

---

## Context

SigmaOS needs a terminal emulator that is fast, supports modern graphics protocols, and integrates with the SigmaOS theme engine.

## Decisions

### 1. Use wgpu for GPU rendering

**Rationale:** wgpu is cross-platform and safe. It gives us a glyph atlas and cell renderer without OpenGL FFI.

### 2. Use alacritty_terminal for VT parsing

**Rationale:** alacritty_terminal is battle-tested, covers the full VT220/xterm-256color spec, and handles the edge cases of escape sequence parsing.

### 3. Config model inspired by Ghostty

**Rationale:** TOML-based, hot-reloadable config with sensible defaults. Users should not need to write a config file at all.

### 4. Sixel and kitty graphics are feature flags

**Rationale:** Both add code complexity. Distro packagers or minimal builds can disable them without losing core terminal functionality.

### 5. PTY abstraction hides OS differences

**Rationale:** On Linux we use `openpty(3)`; future macOS support just needs a different PTY backend without touching the renderer.

## Consequences

- The renderer and PTY are cleanly separated — the renderer never writes to the PTY.
- Color scheme is typed (`Color` struct), not raw strings, to enable theme engine integration.
