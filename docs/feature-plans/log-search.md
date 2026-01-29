# Feature Plan: Log Viewer Search/Find

## Overview

Add search/find capabilities to locate log entries by text or patterns, with navigation between matches. Multiple UX solutions are explored to determine the best workflow for TUI usage.

## Goals

- Search within current file for a query.
- Navigate next/previous matches.
- Optional toggles: regex, case sensitivity, whole word.
- Highlight matches in the list.

## Non-Goals

- Complex filtering by structured fields (future enhancement).

## Solution A: Bottom Bar Search Prompt

### UX Summary

- `Ctrl+F` opens a prompt in the footer bar.
- Flags toggled via shortcuts (e.g., `Alt+C` for case).
- Enter moves to next match; `Shift+Enter` to previous.

### UI Sketch / Demo

See `docs/feature-plans/demos/log-search/bottom-bar-search.ratui` and `docs/feature-plans/demos/log-search/bottom-bar-search.png`.

### Implementation Plan

1. Add `SearchState` with query, flags, match index.
2. When active, render prompt in footer area.
3. On submit, compute match list (line numbers + spans).
4. Jump to match and update highlights.

```rust
pub struct SearchState {
    pub query: String,
    pub regex: bool,
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub matches: Vec<usize>,
    pub selected: usize,
}
```

### Pros / Cons

- ✅ Minimal layout change.
- ✅ Works with current footer bar.
- ❌ Limited space for extra controls.

## Solution B: Modal Search Dialog

### UX Summary

- Modal dialog with toggles and prev/next buttons.
- Keeps controls visible and discoverable.

### UI Sketch / Demo

See `docs/feature-plans/demos/log-search/modal-search.ratui` and `docs/feature-plans/demos/log-search/modal-search.png`.

### Implementation Plan

1. Add `Mode::Search` to state and render modal.
2. Provide toggle buttons or keybinds within modal.
3. Maintain search state and highlight matches.

### Pros / Cons

- ✅ Clear UI for options.
- ✅ Easy to add filters in future.
- ❌ Modal obscures content.

## Solution C: Sidebar Matches List

### UX Summary

- Left sidebar shows search results as line preview list.
- Main area still shows log lines; selecting match jumps.

### UI Sketch / Demo

See `docs/feature-plans/demos/log-search/sidebar-search-results.ratui` and `docs/feature-plans/demos/log-search/sidebar-search-results.png`.

### Implementation Plan

1. Add `search_results_open` boolean and results list state.
2. Split layout for sidebar + main viewer.
3. Selecting match in sidebar updates main list selection.

### Pros / Cons

- ✅ Quick scanning of multiple matches.
- ✅ Works well with multi-file support later.
- ❌ More complex layout logic.

## Demoable Version

Implement Solution A for demo:

- `Ctrl+F` toggles footer prompt.
- Basic substring search and next/previous navigation.
- Highlight current match line.

## Integration Notes

- When partial loading is active, searching should operate on loaded segments first; full search may need background scanning and incremental results.
- Reuse input editor from file import/go-to-line.

## Risks & Open Questions

- Regex performance on large files.
- Whether matches should be precomputed or on-demand.
- How to merge search results with partial loading.

## Validation Checklist

- [ ] Query updates matches list.
- [ ] Next/prev navigation works.
- [ ] Highlights update correctly.
- [ ] Works with large files (partial loading).
