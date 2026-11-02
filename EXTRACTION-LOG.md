# Extraction Log — SigmaOS-terminal

## Source
Extracted from: `SigmaOS-desktop` monorepo, `apps/terminal/` subtree.

## What Was Extracted

| Module | Source path | Notes |
|--------|------------|-------|
| pty.rs | apps/terminal/src/pty.rs | Simplified PTY open/write/resize |
| renderer.rs | apps/terminal/src/renderer.rs | Stub; real wgpu code pending |
| config.rs | apps/terminal/src/config.rs | TOML schema extracted |

## Decisions

1. **Removed winit dependency from lib** — winit is only used in the binary; the lib is renderer-agnostic.
2. **alacritty_terminal** as VT parser — no need to re-implement escape sequence handling.
3. **Sixel/kitty behind feature flags** — kept compile-time optional.

## Known TODOs

- [ ] Real `openpty(3)` call in `Pty::open`
- [ ] wgpu glyph atlas and cell render pipeline
- [ ] Sixel decoder
- [ ] Kitty graphics protocol image upload
- [ ] Regex search over scrollback ring buffer
- [ ] OSC hyperlink parser

## Date
2024-01-01
