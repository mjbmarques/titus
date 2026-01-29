# Feature Plan: Application Color Scheme

## Overview

Define a visually appealing default color scheme for the TUI. The scheme should improve readability, emphasize selected lines, and be compatible with future theme switching.

## Goals

- Provide a cohesive default palette.
- Ensure contrast for accessibility in terminals.
- Highlight key UI areas (header, footer, selection).

## Non-Goals

- Full theme switching (covered separately).

## Solution A: Dracula-Inspired Scheme (current direction)

### UX Summary

- Dark background with purple accents.
- Light text for contrast.

### UI Sketch / Demo

See `docs/feature-plans/demos/color-scheme/dracula-scheme.ratui` and `docs/feature-plans/demos/color-scheme/dracula-scheme.png`.

### Implementation Plan

1. Update `tui/colors.rs` to align with Dracula palette.
2. Use accent colors for headers and selection highlights.
3. Validate contrast in typical terminals.

### Pros / Cons

- ✅ Matches existing palette hints in code.
- ✅ Popular for dev tools.
- ❌ May be too saturated for some users.

## Solution B: Solarized Dark Scheme

### UX Summary

- Muted colors with blue accents.
- Soft contrast for long reading sessions.

### UI Sketch / Demo

See `docs/feature-plans/demos/color-scheme/solarized-dark.ratui` and `docs/feature-plans/demos/color-scheme/solarized-dark.png`.

### Implementation Plan

1. Replace base colors with Solarized palette.
2. Adjust highlights to ensure visible selection.

### Pros / Cons

- ✅ Comfortable for extended use.
- ❌ Lower contrast on some terminals.

## Solution C: Gruvbox Dark Scheme

### UX Summary

- Warm tones and amber accent.

### UI Sketch / Demo

See `docs/feature-plans/demos/color-scheme/gruvbox-dark.ratui` and `docs/feature-plans/demos/color-scheme/gruvbox-dark.png`.

### Implementation Plan

1. Map current colors to Gruvbox base + accent.
2. Use orange accent for active selection.

### Pros / Cons

- ✅ Distinct warm palette.
- ❌ Less common in terminals.

## Demoable Version

Implement Solution A minimal:

- Update `colors.rs` to match Dracula palette.
- Adjust `ui.rs` to use new tokens for header/footer.

## Integration Notes

- Use semantic color tokens (e.g., `background`, `accent`) to prepare for theme support.
- Ensure selection highlight colors meet accessibility contrast.

## Risks & Open Questions

- Whether to allow user override without full theme support.

## Validation Checklist

- [ ] Colors are readable in typical terminal emulators.
- [ ] Highlighted selection is clearly visible.
- [ ] Headers/footers readable at small widths.
