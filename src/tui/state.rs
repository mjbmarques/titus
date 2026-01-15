use ratatui::widgets::ScrollbarState;

#[derive(Debug)]
pub struct State {
    pub scroll_bar_position: ScrollbarState,
    pub ready: bool,
    pub should_quit: bool,
}

impl State {
    pub fn new() -> Self {
        Self { ready: false, should_quit: false, scroll_bar_position: ScrollbarState::default() }
    }
}