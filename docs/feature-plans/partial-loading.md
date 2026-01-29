# Feature Plan: Partial Log File Loading

## Overview

Large log files should not be fully loaded into memory. Implement chunked loading so only necessary portions are kept in RAM while still supporting search and go-to-line features.

## Goals

- Load log files in manageable batches.
- Allow smooth scrolling across segments.
- Provide metadata for total lines and loaded ranges.
- Coordinate with search/go-to-line to load missing ranges.

## Non-Goals

- Full-text indexing (future enhancement).

## Solution A: Windowed Cache (Sliding Window)

### UX Summary

- Load a window of N lines around the current selection.
- Scrolling near edges loads next/previous window.

### UI Sketch / Demo

See `docs/feature-plans/demos/partial-loading/windowed-view.ratui` and `docs/feature-plans/demos/partial-loading/windowed-view.png`.

### Implementation Plan

1. Introduce `FileBatch { lower, upper }` window state (already exists).
2. Add `loaded_ranges: Vec<FileBatch>` cache with max size.
3. When selection reaches edge threshold, trigger `LoadFileRange` task.
4. Evict least-recently-used batches.

```rust
pub struct LogFileState {
    pub file_loaded_lines: FileBatch,
    pub loaded_ranges: Vec<FileBatch>,
}
```

### Pros / Cons

- ✅ Predictable memory usage.
- ✅ Easy to implement with existing batching.
- ❌ Jumping far distances requires repeated loads.

## Solution B: Infinite Scroll Buffer

### UX Summary

- Always append/prepend new chunks as user scrolls.
- Keep a buffer size and discard oldest lines when exceeding size.

### UI Sketch / Demo

See `docs/feature-plans/demos/partial-loading/infinite-scroll.ratui` and `docs/feature-plans/demos/partial-loading/infinite-scroll.png`.

### Implementation Plan

1. Maintain `VecDeque<String>` for loaded lines.
2. Track logical line offset to map to file line numbers.
3. Load more lines when near top/bottom.

### Pros / Cons

- ✅ Smooth continuous scrolling.
- ✅ Natural with scroll wheel.
- ❌ Harder to coordinate with search/goto.

## Solution C: Segment Navigation

### UX Summary

- File divided into segments; user navigates segments explicitly.
- Segment shown in header for clarity.

### UI Sketch / Demo

See `docs/feature-plans/demos/partial-loading/segment-nav.ratui` and `docs/feature-plans/demos/partial-loading/segment-nav.png`.

### Implementation Plan

1. Precompute total segments by file length.
2. Provide controls to move segments (Prev/Next).
3. Update list with new segment content.

### Pros / Cons

- ✅ Simple mental model.
- ✅ Works well with multi-file support.
- ❌ Less smooth; adds manual step.

## Demoable Version

Implement Solution A minimal:

- Load initial window (e.g., 400 lines).
- On scroll beyond threshold, load next/prev window.
- Display loaded range in header.

## Integration Notes

- Search should operate on currently loaded window and optionally trigger background scans.
- Go-to-line should load a window centered on target line.

## Risks & Open Questions

- How to efficiently count total lines (maybe lazy counting)?
- Whether to store offsets for quick seeks.

## Validation Checklist

- [ ] Memory stays bounded for large files.
- [ ] Scrolling loads new chunks seamlessly.
- [ ] Go-to-line triggers window load.
- [ ] Search uses loaded content and can expand.
