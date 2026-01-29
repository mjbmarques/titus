# Feature Plan: Go-to Line

## Overview

Provide a quick jump mechanism to move the log viewer to a specific line number. The flow should be consistent with other input-driven features (search, file import) and should handle very large files with partial loading.

## Goals

- Accept a line number input and move focus to that line.
- Provide validation for out-of-range or invalid inputs.
- Make it easy to open/close and keep existing navigation intact.

## Non-Goals

- Implementing advanced bookmarking (separate feature).

## Solution A: Modal Input Dialog

### UX Summary

- Press `g` to open modal.
- Enter line number, press `Enter` to jump.

### UI Sketch / Demo

See `docs/feature-plans/demos/go-to-line/modal-input.ratui`.

### Implementation Plan

1. Add `goto_line` state (input string, parsed value, error).
2. Toggle `Mode::GoToLine` (new mode) to show dialog.
3. On submit, validate: numeric, > 0, <= max line.
4. Update list offset and selection to target line.

```rust
pub struct GoToLineState {
    pub input: String,
    pub error: Option<String>,
}
```

### Pros / Cons

- ✅ Consistent with file import modal.
- ✅ Easy validation messaging.
- ❌ Modal interrupts reading flow.

## Solution B: Inline Jump Bar

### UX Summary

- A horizontal bar appears above the list when activated.
- Shows range and last jump value.

### UI Sketch / Demo

See `docs/feature-plans/demos/go-to-line/inline-jumpbar.ratui`.

### Implementation Plan

1. Add `goto_line_open` boolean to state.
2. Split layout to render jump bar when open.
3. When Enter pressed, validate and jump.

### Pros / Cons

- ✅ Lower cognitive load; stays in context.
- ❌ Uses vertical space.

## Solution C: Status Bar Prompt

### UX Summary

- Reuse footer area to accept input while keeping view intact.

### UI Sketch / Demo

See `docs/feature-plans/demos/go-to-line/status-prompt.ratui`.

### Implementation Plan

1. Reuse footer area to show `Go to line:` prompt.
2. Capture typed digits; update prompt dynamically.
3. On Enter, validate and jump.

### Pros / Cons

- ✅ Minimal layout changes.
- ❌ Less space for instructions while active.

## Demoable Version

Implement Solution C as a minimal demo:

- Press `g` to enable status prompt.
- Type digits; press Enter to jump to line.
- Esc cancels.

## Integration Notes

- When partial loading is enabled, jumping to a line should trigger a file batch load (if needed) before updating the list.
- Reuse input editor logic from search and file import.

## Risks & Open Questions

- How to represent total line count if only part of the file is loaded?
- Should out-of-range jumps clamp or error?

## Validation Checklist

- [ ] Input accepts only digits and handles backspace.
- [ ] Jump moves selection and scroll offset.
- [ ] Out-of-range input shows feedback.
- [ ] Works when partial loading is active.

