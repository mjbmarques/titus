use crate::tui::state::{Mode, State, ViewType};
use ratatui::crossterm::event::{KeyEvent, KeyEventKind};
use crate::tui::widgets::text_input_cursor::TextInputCursor;

mod viewer_keybinds;
mod find_command_keybinds;

// Try keybinds and map them depending on the mode.
// For now there is only one mode so no mapping is done, temporarily.
pub fn try_keybinds(state: &mut State, event: KeyEvent) {
    let mode = state.current_mode.borrow().mode.clone();
    match mode {
        Mode::Viewer() => try_viewer_keybinds(state, event),
        Mode::Find(mut input, view_type) => {
            try_find_command_keybinds(state, event, &mut input, view_type.clone());
            if matches!(state.current_mode.borrow().mode, Mode::Find(_, _)) {
                state.current_mode.borrow_mut().mode = Mode::Find(input, view_type);
            }
        }
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

fn try_find_command_keybinds(state: &mut State, event: KeyEvent, input: &mut TextInputCursor, view_type: ViewType) {
    match event.kind {
        KeyEventKind::Press => find_command_keybinds::try_keypress(state, event, input, view_type),
        KeyEventKind::Release => find_command_keybinds::try_keypress(state, event, input, view_type),
        KeyEventKind::Repeat => find_command_keybinds::try_keypress(state, event, input, view_type),
    }
}

fn try_go_to_line_command_keybinds(state: &mut State, key_event: KeyEvent) {
    todo!()
}

fn try_file_import_command_keybinds(state: &mut State, key_event: KeyEvent) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::{KeyCode, KeyModifiers};
    use std::sync::mpsc;

    #[test]
    fn try_keybinds_updates_find_mode_input_text() {
        let (sender, _) = mpsc::channel();
        let mut state = State::new(sender);
        state.current_mode.borrow_mut().mode = Mode::Find(TextInputCursor::new_empty(), ViewType::Command);

        try_keybinds(&mut state, KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));

        match state.current_mode.borrow().mode.clone() {
            Mode::Find(input, _) => assert_eq!(input.input_text, "a"),
            _ => panic!("expected find mode"),
        }
    }
}
