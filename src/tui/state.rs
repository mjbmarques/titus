use ratatui::widgets::ScrollbarState;
use crate::tui::widgets::list::SelectableList;

#[derive(Debug)]
pub struct State {
    pub scroll_bar_position: ScrollbarState,
    pub ready: bool,
    pub should_quit: bool,
    // aside form curr_open_file, another field can be created as a list later on to represent
    // multiple open files.
    pub curr_open_file: Option<LogFileState>,
    pub current_list: SelectableList<String>,
    pub recommended_list_offset: usize,
}

pub enum Mode {
    // When navigating through the file logs.
    Viewer,
    // When viewing a line's details.
    Details,
    // When in search/find dialog.
    Search,
    // When in file import dialog.
    FileImport,
}

#[derive(Debug, Clone)]
pub struct FileBatch {
    pub upper: usize,
    pub lower: usize
}

#[derive(Debug, Clone)]
pub struct LogFileState {
    pub id: usize,
    pub file_name: String,
    pub file_path: String,
    pub file_lines: usize,
    pub file_loaded_lines: FileBatch,
    pub loaded_lines: SelectableList<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            ready: false,
            should_quit: false,
            scroll_bar_position: ScrollbarState::default(),
            curr_open_file: None,
            current_list: SelectableList::default(),
            recommended_list_offset: 0,
        }
    }
}