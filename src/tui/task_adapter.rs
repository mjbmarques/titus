use crate::tui::state::{FileBatch, LogFileState, State};
use crate::tui::widgets::list::SelectableList;

pub struct TaskAdapter {
    log_file_id: usize,
}

impl TaskAdapter {
    pub fn new() -> Self {
        Self { log_file_id: 0 }
    }
    pub fn file_content(
        &mut self,
        state: &mut State,
        file_location: String,
        file_content: Vec<String>,
    ) {
        handle_file_content(self, state, file_location, file_content);
    }
}
pub fn handle_file_content(
    task_adapter: &mut TaskAdapter,
    state: &mut State,
    file_location: String,
    file_content: Vec<String>,
) {
    task_adapter.log_file_id += 1;

    let log_file = LogFileState {
        id: task_adapter.log_file_id,
        file_name: file_location.clone(),
        file_path: file_location.clone(),
        file_lines: file_content.len(),
        file_loaded_lines: FileBatch {
            upper: file_content.len().saturating_sub(1),
            lower: 0,
        },
        loaded_lines: SelectableList::with_items(file_content),
    };
    state.curr_open_file = Some(log_file.clone());

    state.current_list = log_file.clone().loaded_lines;
}
