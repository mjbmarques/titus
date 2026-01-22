use ratatui::widgets::ScrollbarState;
use crate::tui::file_io;
use crate::tui::widgets::list::SelectableList;

#[derive(Debug)]
pub struct State {
    pub scroll_bar_position: ScrollbarState,
    pub scroll_offset: usize,
    pub page_size: usize,
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
            scroll_offset: 0,
            page_size: 0,
            curr_open_file: None,
            item_list: SelectableList::default(),
        }
    }

    pub fn load_log_lines(&mut self, path: &str) {
        let mut lines_vec: Vec<String> = Vec::new();
        if let Ok(lines) = file_io::read_lines(path) {
            for line in lines.map_while(Result::ok) {
                lines_vec.push(line);
            }
        }
        self.item_list = SelectableList::with_items(lines_vec);
        self.curr_open_file = if self.item_list.items.is_empty() {
            None
        } else {
            Some(path.to_string())
        };
        self.scroll_offset = 0;
        self.scroll_bar_position = ScrollbarState::new(self.item_list.items.len());
    }
}
