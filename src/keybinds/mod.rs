use ratatui::crossterm::event::{KeyEvent, KeyEventKind};
use crate::tui::state::State;

pub mod default_mode;

// Try keybinds and map them depending on the mode.
// For now there is only one mode so no mapping is done, temporarily.
pub fn try_keybinds(state: &mut State, event: KeyEvent) {
    match event.kind {
        KeyEventKind::Press => default_mode::try_keypress(state, event),
        KeyEventKind::Release => default_mode::try_key_release(state, event),
        KeyEventKind::Repeat => default_mode::try_key_repeat(state, event)
    }
}