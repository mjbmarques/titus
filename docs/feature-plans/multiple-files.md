# Feature Plan: Multiple Files Support

## Overview

Enable the log viewer to open and switch between multiple files in the same session. This is a foundational feature for workflows that compare logs or monitor multiple sources.

## Goals

- Allow multiple files to remain open.
- Provide a clear UI to switch active file.
- Preserve scroll position per file.

## Non-Goals

- Simultaneous side-by-side diffing (future enhancement).

## Solution A: Tab Bar

### UX Summary

- Tabs at the top show open file names.
- `Alt+Tab` cycles through tabs to avoid conflict with close-tab conventions.

### UI Sketch / Demo

See `docs/feature-plans/demos/multiple-files/tab-bar.ratui` and `docs/feature-plans/demos/multiple-files/tab-bar.png`.

### Implementation Plan

1. Extend `State` with `open_files: Vec<LogFileState>` and `active_file_index`.
2. Render tab bar in header area.
3. Use keybinds to cycle/close tabs.

### Pros / Cons

- ✅ Familiar UI pattern.
- ✅ Works with many files.
- ❌ Long filenames may truncate.

## Solution B: Sidebar File Switcher

### UX Summary

- Left sidebar lists open files; selection changes active file.

### UI Sketch / Demo

See `docs/feature-plans/demos/multiple-files/sidebar-switcher.ratui` and `docs/feature-plans/demos/multiple-files/sidebar-switcher.png`.

### Implementation Plan

1. Split layout into sidebar + viewer.
2. Render list of open files with selection.
3. Use `Tab` to focus file list.

### Pros / Cons

- ✅ Great for long file names.
- ✅ Plays well with search sidebar if combined.
- ❌ Uses screen real estate.

## Solution C: Dropdown Switcher

### UX Summary

- Dropdown in header to select active file.

### UI Sketch / Demo

See `docs/feature-plans/demos/multiple-files/dropdown-switcher.ratui` and `docs/feature-plans/demos/multiple-files/dropdown-switcher.png`.

### Implementation Plan

1. Add dropdown state (open/close).
2. Render current file name + dropdown arrow.
3. Selecting entry swaps active file.

### Pros / Cons

- ✅ Compact UI.
- ❌ Less discoverable; extra steps to switch.

## Demoable Version

Implement Solution A minimal:

- Render tab bar with active highlight.
- Add keybind `Alt+Tab` to cycle files.
- Maintain per-file scroll positions.

## Integration Notes

- Partial loading needs per-file batches.
- Search state should reset or be scoped per file.
- Go-to-line should operate on active file only.

## Risks & Open Questions

- How many files can be open before performance degrades?
- How to handle file-specific errors or reloads?

## Validation Checklist

- [ ] Switch between files retains scroll positions.
- [ ] Active file updates header and list content.
- [ ] Closing files selects a sensible next file.
