# Analysis

## Bug root cause

In `src/keybinds/mod.rs`, `try_keybinds` matched on a cloned `Mode`:

- `Mode::Find(mut input, view_type)` gave a mutable local copy of `TextInputCursor`.
- `find_command_keybinds::try_keypress` updated that local copy.
- The updated value was not written back to `state.current_mode`.
- `src/tui/ui.rs` reads from `state.current_mode`, so typed characters were not displayed.

## Rc / RefCell review

Current usage centers on:

- `State.current_mode: Rc<RefCell<ModeState>>`
- `State.all_modes: Vec<Rc<RefCell<ModeState>>>`

This is workable for shared mutable mode management, but it increases risk of clone-vs-shared-state mistakes when mode payloads carry mutable data (`TextInputCursor`).

Safer alternatives (future work, not part of this minimal fix):

1. Store command input buffers directly in `State` fields and keep `Mode` as a lightweight discriminator.
2. Keep `Mode` payloads, but avoid clone-driven mutation paths and mutate mode through explicit `borrow_mut` flows.
3. If multi-owner mutable state is not needed, prefer plain owned structs over `Rc<RefCell<_>>`.
