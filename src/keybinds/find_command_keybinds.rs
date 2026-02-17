use std::rc::Rc;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::state::{Mode, State};

pub fn try_keypress(state: &mut State, event: KeyEvent) {
    if KeyModifiers::CONTROL == event.modifiers {
        try_ctrl_mod_keypress(state, event);
        return;
    }

    match event.code {
        KeyCode::Enter => submit_find(state),
        KeyCode::Char(to_insert) => enter_char(state, to_insert),
        KeyCode::Backspace => delete_char_left(state),
        KeyCode::Delete => delete_char_right(state),
        KeyCode::Left => move_cursor_left(state),
        KeyCode::Right => move_cursor_right(state),
        KeyCode::Esc => back_to_view(state),
        _ => {}
    }
}

fn enter_char(state: &mut State, to_insert: char) {
    if let Mode::Find(input, _) = &mut state.current_mode.borrow_mut().mode {
        input.enter_char(to_insert);
    }
}

fn delete_char_left(state: &mut State) {
    if let Mode::Find(input, _) = &mut state.current_mode.borrow_mut().mode {
        input.delete_char_left();
    }
}

fn delete_char_right(state: &mut State) {
    if let Mode::Find(input, _) = &mut state.current_mode.borrow_mut().mode {
        input.delete_char_right();
    }
}

fn move_cursor_left(state: &mut State) {
    if let Mode::Find(input, _) = &mut state.current_mode.borrow_mut().mode {
        input.move_cursor_left();
    }
}

fn move_cursor_right(state: &mut State) {
    if let Mode::Find(input, _) = &mut state.current_mode.borrow_mut().mode {
        input.move_cursor_right();
    }
}

fn submit_find(state: &mut State) {
    todo!("not implemented")
}

fn back_to_view(state: &mut State) {
    let view_mode = state.all_modes
        .iter()
        .find(|mode| mode.borrow().mode == Mode::Viewer());
    match view_mode {
        None => {}
        Some(mode) => state.current_mode = Rc::clone(mode),
    }
}

fn try_ctrl_mod_keypress(state: &mut State, event: KeyEvent) {
    match event.code {
        KeyCode::Char('f') => back_to_view(state),
        _ => log::warn!("Unhandled CTRL+key in Find mode: {:?}", event),
    }
}

pub fn try_key_release(_: &mut State, _: KeyEvent) {
}

pub fn try_key_repeat(state: &mut State, event: KeyEvent) {
    try_keypress(state, event);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::widgets::text_input_cursor::TextInputCursor;
    use crate::tui::state::{ModeState, ViewType};
    use ratatui::crossterm::event::{KeyEventKind, KeyEventState};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::mpsc;

    fn setup_find_state() -> State {
        let (sender, _) = mpsc::channel();
        let mut state = State::new(sender);
        state.current_mode = Rc::new(RefCell::new(ModeState::new(Mode::Find(
            TextInputCursor::new_empty(),
            ViewType::Command,
        ))));
        state
    }

    #[test]
    fn try_key_repeat_enters_char_in_find_mode() {
        let mut state = setup_find_state();
        let repeat_char_event = KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Repeat,
            state: KeyEventState::NONE,
        };

        try_key_repeat(&mut state, repeat_char_event);

        if let Mode::Find(input, _) = &state.current_mode.borrow().mode {
            assert_eq!(input.input_text, "a");
        } else {
            panic!("state should be in find mode");
        }
    }
}
