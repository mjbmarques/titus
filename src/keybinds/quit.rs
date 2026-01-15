use log::debug;
use ratatui::crossterm::event::KeyEvent;
use crate::tui::state::State;

pub fn try_key(state: &mut State, event: KeyEvent) {

        if event.code.is_char('q') || event.code.is_esc() {
            debug!("Executing the quit keybind.");
            state.should_quit = true;
        }
}