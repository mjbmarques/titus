use std::rc::Rc;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::state::{Mode, State, ViewType};
use crate::tui::widgets::text_input_cursor::TextInputCursor;

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

pub fn try_key_release(state: &mut State, event: KeyEvent) {
    log::warn!("Unhandled release keypress in Find mode: {:?}", event);
}

pub fn try_key_repeat(state: &mut State, event: KeyEvent) {
    log::warn!("Unhandled repeat keypress in Find mode: {:?}", state);
}