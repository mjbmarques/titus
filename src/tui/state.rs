use ratatui::widgets::ScrollbarState;
use crate::tui::widgets::list::SelectableList;

#[derive(Debug)]
pub struct State {
    pub scroll_bar_position: ScrollbarState,
    pub ready: bool,
    pub should_quit: bool,
    pub curr_open_file: Option<String>,
    pub item_list: SelectableList<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            ready: false,
            should_quit: false,
            scroll_bar_position: ScrollbarState::default(),
            curr_open_file: None,
            item_list: SelectableList::default(),
        }
    }
}