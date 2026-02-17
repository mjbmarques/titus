use crate::tui::state::{Mode, State};
use ratatui::crossterm::event::{KeyEvent, KeyEventKind};

mod viewer_keybinds;
mod find_command_keybinds;

// Try keybinds and map them depending on the mode.
// For now there is only one mode so no mapping is done, temporarily.
pub fn try_keybinds(state: &mut State, event: KeyEvent) {
    let mode = state.current_mode.borrow().mode.clone();
    match mode {
        // TODO: in the future make this able to call the try keybinds with the parameters from the
        //  Find(input, view_type) and etc...
        Mode::Viewer() => try_viewer_keybinds(state, event),
        Mode::Find(_, _) => try_find_command_keybinds(state, event),
        Mode::GoToLine(_, _) => try_go_to_line_command_keybinds(state, event),
        Mode::FileImport(_, _) => try_file_import_command_keybinds(state, event),
        Mode::Undefined | Mode::Details => { /* No keybinds for these modes yet */ }
    }
}

fn try_viewer_keybinds(state: &mut State, event: KeyEvent) {
    match event.kind {
        KeyEventKind::Press => viewer_keybinds::try_keypress(state, event),
        KeyEventKind::Release => viewer_keybinds::try_key_release(state, event),
        KeyEventKind::Repeat => viewer_keybinds::try_key_repeat(state, event),
    }
}

fn try_find_command_keybinds(state: &mut State, event: KeyEvent) {
    match event.kind {
        KeyEventKind::Press => find_command_keybinds::try_keypress(state, event),
        KeyEventKind::Release => find_command_keybinds::try_key_release(state, event),
        KeyEventKind::Repeat => find_command_keybinds::try_key_repeat(state, event),
    }
}

fn try_go_to_line_command_keybinds(state: &mut State, key_event: KeyEvent) {
    todo!()
}

fn try_file_import_command_keybinds(state: &mut State, key_event: KeyEvent) {
    todo!()
}
