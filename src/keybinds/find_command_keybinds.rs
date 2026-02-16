use std::rc::Rc;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::state::{Mode, State, ViewType};
use crate::tui::widgets::text_input_cursor::TextInputCursor;

pub fn try_keypress(state: &mut State, event: KeyEvent, input: &mut TextInputCursor, view_type: ViewType) {
    if KeyModifiers::CONTROL == event.modifiers {
        try_ctrl_mod_keypress(state, event);
        return;
    }

    match event.code {
        KeyCode::Enter => submit_find(state),
        KeyCode::Char(to_insert) => input.enter_char(to_insert),
        KeyCode::Backspace => input.delete_char(),
        KeyCode::Left => input.move_cursor_left(),
        KeyCode::Right => input.move_cursor_right(),
        KeyCode::Esc => back_to_view(state),
        _ => {}
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

}