# Scrollable Log View

## Decisions
- **Scroll UX:** selectable rows with highlight; scrolling follows selection.
- **Keys:** Up/Down move one line; PageUp/PageDown move one viewport.
- **Bounds:** clamped at the top and bottom (no wrap).
- **Ownership:** log lines are stored in `State` to avoid rebuilding data each render and to keep borrowing simple in Rust.

## What was implemented
### State ownership and scroll bookkeeping
- Added `scroll_offset` and `page_size` to `State`.
- Added `State::load_log_lines(path)` to load log file lines once and store them in `item_list`.

### Scroll-aware rendering
- The log view now renders only a viewport slice based on `scroll_offset` and terminal height.
- Selection stays visible by adjusting `scroll_offset` when the selected row moves out of view.
- The footer now displays PageUp/PageDown keys.

### Key handling
- Added a `keybinds::scroll` module to handle Up/Down/PageUp/PageDown.
- Wired scroll key handling into the main event loop.

### Selection clamping
- `SelectableList` selection now clamps at bounds and handles empty lists safely.

## Files changed
- `src\tui\state.rs`
- `src\tui\ui.rs`
- `src\tui\widgets\list.rs`
- `src\keybinds\scroll.rs` (new)
- `src\keybinds\mod.rs`
- `src\lib.rs`

## Notes for future work
- The log file path is still hardcoded; consider loading from configuration or a file picker.
- A scrollbar can be added using `Scrollbar` and `ScrollbarState` if desired.
