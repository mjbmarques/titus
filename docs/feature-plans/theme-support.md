# Feature Plan: Application Theme Support

## Overview

Add multiple selectable themes so users can switch palettes without modifying the code. Themes should be compatible with the default color scheme and future UI components.

## Goals

- Support at least one additional theme (e.g., Catppuccin).
- Allow switching themes at runtime or via config.
- Keep color usage semantic (background, accent, warning).

## Non-Goals

- Full plugin system for third-party themes.

## Solution A: Theme Selector Modal

### UX Summary

- `Ctrl+T` opens a modal with theme list.
- User selects and applies theme.

### UI Sketch / Demo

See `docs/feature-plans/demos/theme-support/theme-selector.ratui`.

### Implementation Plan

1. Define `Theme` struct with semantic colors.
2. Add `ThemeManager` to state.
3. Render modal and update theme on selection.

```rust
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub warning: Color,
}
```

### Pros / Cons

- ✅ User-friendly selection UI.
- ❌ Adds modal complexity.

## Solution B: Config-Driven Theme

### UX Summary

- Theme is loaded from config file on startup.
- Config includes palette definitions.

### UI Sketch / Demo

See `docs/feature-plans/demos/theme-support/config-file.ratui`.

### Implementation Plan

1. Add `config/themes.yaml` with palette definitions.
2. Parse config on startup; apply `current` theme.
3. Keep fallback to default if config missing.

### Pros / Cons

- ✅ Works without runtime UI changes.
- ✅ Easy to distribute theme packs.
- ❌ Requires restart to apply.

## Solution C: Runtime Preview Panel

### UX Summary

- Sidebar preview showing themes with live preview.
- Press Enter to apply.

### UI Sketch / Demo

See `docs/feature-plans/demos/theme-support/runtime-preview.ratui`.

### Implementation Plan

1. Add theme preview panel in layout.
2. Selecting theme updates UI instantly for preview.
3. Confirm selection to persist.

### Pros / Cons

- ✅ Visual comparison of themes.
- ❌ Complex UI layout.

## Demoable Version

Implement Solution A minimal:

- Static list of themes.
- Apply theme to UI tokens at runtime.

## Integration Notes

- Color scheme defaults should map to Theme struct values.
- Search, go-to-line, and file import overlays should use semantic colors.

## Risks & Open Questions

- Where to persist theme selection (config vs. memory)?
- How to ensure contrast across all themes?

## Validation Checklist

- [ ] Theme can be switched at runtime.
- [ ] All UI areas update to new theme.
- [ ] Config fallback works.

