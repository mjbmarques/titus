use crate::tui::state::State;
use log::debug;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
                //--------------Test keybinds----------------
                Some('0') => state.current_list.force_select_with_offset(0, 0),
                Some('1') => state.current_list.force_select_with_offset(1, 1),
                Some('2') => state.current_list.force_select_with_offset(2, 2),
                Some('3') => state.current_list.force_select_with_offset(3, 3 - 2),
                Some('4') => state.current_list.force_select_with_offset(4, 4 - 2),
                Some('5') => state.current_list.force_select_with_offset(5, 5 - 2),
                Some('6') => state.current_list.force_select_with_offset(6, 6 - 1),
                Some('7') => state.current_list.force_select_with_offset(7, 7 - 1),
                Some('8') => state.current_list.force_select_with_offset(8, 8 - 4),
                Some('9') => state.current_list.force_select_with_offset(9, 9 - 4),
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
