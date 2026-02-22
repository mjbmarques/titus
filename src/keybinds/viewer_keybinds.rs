use crate::tui::state::{Mode, ModeState, State, ViewType};
use crate::tui::widgets::text_input_cursor::TextInputCursor;
use log::debug;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cell::RefCell;
use std::rc::Rc;

pub fn try_keypress(state: &mut State, event: KeyEvent) {
    if KeyModifiers::CONTROL == event.modifiers {
        try_ctrl_mod_keypress(state, event);
        return;
    }
    match event.code {
        KeyCode::Up => select_prev(state),
        KeyCode::Down => select_next(state),
        KeyCode::Left => state.current_list.decrease_horizontal_offset(1),
        KeyCode::Right => state.current_list.increase_horizontal_offset(1),
        // KeyCode::Left => state.current_list.force_select_first(),
        // KeyCode::Right => state.current_list.force_select_last(),
        // KeyCode::PageUp => state.item_list.select_prev(state.page_size.max(1)),
        // KeyCode::PageDown => state.item_list.select_next(state.page_size.max(1)),
        KeyCode::Char(c) => {
            match Some(c) {
                // -----------Production keybinds-------------
                Some('q') => quit(state),
                _ => log::warn!("Unhandled PRESS char key: {:?}", event),
            }
        }
        KeyCode::Esc => quit(state),
        _ => log::warn!("Unhandled PRESS key: {:?}", event),
    }
}

fn try_ctrl_mod_keypress(state: &mut State, event: KeyEvent) {
    match event.code {
        // KeyCode::Up => state.current_list.decrement_offset(1),
        // KeyCode::Down => state.current_list.increment_offset(1),
        KeyCode::Up => state.current_list.state.scroll_up_by(1),
        KeyCode::Down => state.current_list.state.scroll_down_by(1),
        KeyCode::Char('f') => find_clicked(state),
        _ => log::warn!("Unhandled PRESS CTRL+key: {:?}", event),
    }
}

pub fn try_key_release(_: &mut State, event: KeyEvent) {
    log::warn!("Unhandled RELEASE key: {:?}", event);
}

pub fn try_key_repeat(state: &mut State, event: KeyEvent) {
    match event.code {
        KeyCode::Up => select_prev(state),
        KeyCode::Down => select_next(state),
        _ => log::warn!("Unhandled REPEAT key: {:?}", event),
    }
}

fn find_clicked(state: &mut State) {
    let find_mode = Rc::new(RefCell::new(ModeState::new(Mode::Find(
        TextInputCursor::new_empty(),
        ViewType::Command,
    ))));
    state.current_mode = find_mode.clone();
    let exists_find_mode = state.all_modes.iter().any(|mode| {
        let mode_ref = mode.borrow();
        matches!(mode_ref.mode, Mode::Find(_, ViewType::Command))
    });
    if !exists_find_mode {
        state.all_modes.push(state.current_mode.clone());
    }
}

fn quit(state: &mut State) {
    debug!("Executing the quit keybind.");
    state.should_quit = true;
}

fn select_next(state: &mut State) {
    debug!("Selecting the next item in the list.");
    state.current_list.select_next(1);
}

fn select_prev(state: &mut State) {
    debug!("Selecting the previous item in the list.");
    state.current_list.select_prev(1);
}
