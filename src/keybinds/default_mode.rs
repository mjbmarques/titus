use log::debug;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::tui::state::State;

pub fn try_keypress(state: &mut State, event: KeyEvent) {
    match event.code {
        KeyCode::Up => select_prev(state),
        KeyCode::Down => select_next(state),
        // KeyCode::PageUp => state.item_list.select_prev(state.page_size.max(1)),
        // KeyCode::PageDown => state.item_list.select_next(state.page_size.max(1)),
        KeyCode::Esc => quit(state),
        KeyCode::Char('q') => quit(state),
        _ => log::warn!("Unhandled PRESS key: {:?}", event),
    }
}
pub fn try_key_release(state: &mut State, event: KeyEvent) {
    log::warn!("Unhandled RELEASE key: {:?}", event);
}

pub fn try_key_repeat(state: &mut State, event: KeyEvent) {
    match event.code {
        KeyCode::Up => select_prev(state),
        KeyCode::Down => select_next(state),
        _ => log::warn!("Unhandled REPEAT key: {:?}", event)
    }
}

fn quit(state: &mut State) {
    debug!("Executing the quit keybind.");
    state.should_quit = true;
}

fn select_next(state: &mut State) {
    debug!("Selecting the next item in the list.");
    state.item_list.select_next(1);
}

fn select_prev(state: &mut State) {
    debug!("Selecting the previous item in the list.");
    state.item_list.select_prev(1);
}