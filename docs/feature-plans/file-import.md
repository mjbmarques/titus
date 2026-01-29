# Feature Plan: File Import

## Overview

Enable users to import a log file interactively from the TUI. CLI arguments already load a file; this feature adds a dialog/flow for file selection, validation, and feedback. The plan includes multiple UI solutions so we can pick the best experience.

## Goals

- Allow interactive file selection without restarting the app.
- Provide clear validation errors for invalid paths or unreadable files.
- Keep the log viewer responsive while files load.
- Ensure the design works with other planned features (search, theming, multi-file).

## Non-Goals

- Replace CLI argument support.
- Implement multi-file switching (handled in separate feature).

## Solution A: Modal File Import Dialog

### UX Summary

- A centered modal dialog appears on `Ctrl+O`.
- Input field for file path, optional browse list, and primary actions.
- Validation errors appear below the input.

### UI Sketch / Demo

See `docs/feature-plans/demos/file-import/modal-dialog.ratui`.

### Implementation Plan

1. **State additions**
   - Extend `State` with `file_import` UI state (input text, validation error, focus).
   - Add `Mode::FileImport` usage to toggle modal visibility.
2. **Key handling**
   - Add keybind in `keybinds::try_keybinds` to open modal (`Ctrl+O`).
   - While in `Mode::FileImport`, route keystrokes to the input editor and actions.
3. **UI rendering**
   - Render modal when `Mode::FileImport` is active.
   - Use `Paragraph` for input, `Block` for border, `Layout` to center.
4. **Validation and task dispatch**
   - On submit, validate path (exists, file, readable) via file IO check.
   - If valid, trigger `EventRequest::LoadFile` and close modal.
   - If invalid, set error message for display.

### Implementation Notes

```rust
pub struct FileImportState {
    pub input: String,
    pub error: Option<String>,
    pub cursor: usize,
}
```

```rust
if matches!(state.current_mode.mode, Mode::FileImport) {
    render_file_import_modal(frame, state);
}
```

### Pros / Cons

- ✅ Familiar UX for dialogs, minimal screen disruption.
- ✅ Easy to add validation messaging.
- ❌ Modal interrupts workflow; no browsing without typing.

## Solution B: Inline Import Panel

### UX Summary

- A collapsible panel at top of log view with input and actions.
- Panel slides in/out with `Ctrl+O`.

### UI Sketch / Demo

See `docs/feature-plans/demos/file-import/inline-form.ratui`.

### Implementation Plan

1. Add `file_import_panel_open` boolean to state.
2. Add a layout row above the list when panel open.
3. Provide `Ctrl+O` to toggle, `Enter` to load, `Esc` to close.
4. Use same validation and task dispatch as modal.

### Pros / Cons

- ✅ Keeps focus in context of log list.
- ✅ Easy to show quick path history.
- ❌ Uses vertical space; can clutter smaller terminals.

## Solution C: Sidebar File Browser

### UX Summary

- Left sidebar with tree list of recent directories.
- Use arrow keys to navigate and select files.

### UI Sketch / Demo

See `docs/feature-plans/demos/file-import/sidebar-browser.ratui`.

### Implementation Plan

1. Add a `file_browser` component to state with tree nodes.
2. Split main layout into sidebar + log view.
3. Add `Tab` to switch focus between sidebar and log list.
4. On selection, trigger `LoadFile` and collapse sidebar.

### Pros / Cons

- ✅ Discoverable for non-CLI users.
- ✅ Works well with multi-file support later.
- ❌ More complex state management.

## Demoable Version

For demo purposes, implement a lightweight stub that:

- Opens a modal on `Ctrl+O` (Solution A).
- Accepts input and shows validation error if empty.
- On success, dispatches `LoadFile`.

## Integration Notes

- Coordinate with search/find overlay so modals are layered correctly.
- Use a shared input component for Go-to-line, search, and file import.
- Consider theme tokens for dialog borders and error colors.

## Risks & Open Questions

- Should file paths be validated synchronously or via background task?
- Should there be a recent files list? (links to multi-file support)
- How to unify modal styling with future theming?

## Validation Checklist

- [ ] Modal opens/closes on keybind.
- [ ] Invalid paths show errors.
- [ ] Valid file dispatches load task.
- [ ] Logs remain responsive during load.

