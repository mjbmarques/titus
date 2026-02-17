# Implementation Decisions

## Decision 1: minimal bug fix in keybind dispatch

Updated `src/keybinds/mod.rs` so that after handling a find-mode key event, the modified `TextInputCursor` is written back into `state.current_mode` when the mode is still `Find`.

Why:

- Fixes the lost-update bug with minimal scope.
- Preserves existing architecture and mode variants.
- Avoids larger refactors to mode/state ownership in this issue.

## Decision 2: preserve mode switch behavior

Write-back is conditional (`Mode::Find` still active) to avoid undoing transitions such as `Esc` returning to viewer mode.

## Decision 3: targeted regression test

Added a unit test in `src/keybinds/mod.rs` that:

- Places state in find mode.
- Sends a `Char('a')` key event through `try_keybinds`.
- Verifies `state.current_mode` now contains input text `"a"`.
