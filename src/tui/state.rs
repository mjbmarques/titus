use std::cell::RefCell;
use std::rc::Rc;
use crate::tui::event::{EventResponse, TaskScheduler};
use crate::tui::tui_event::TuiEvent;
use crate::tui::widgets::list::SelectableList;
use ratatui::widgets::ScrollbarState;
use std::sync::mpsc::Sender;
use crate::tui::widgets::text_input_cursor::TextInputCursor;

#[derive(Debug)]
pub struct State {
    pub current_mode: Rc<RefCell<ModeState>>,
    pub all_modes: Vec<Rc<RefCell<ModeState>>>, // TODO: implement mode switching and management.
    pub task_scheduler: TaskScheduler,
    pub scroll_bar_position: ScrollbarState,
    pub ready: bool,
    pub should_quit: bool,
    // TODO: consider just using index to all_open_files.
    //  might or not be a good idea.
    pub curr_open_file: Option<LogFileState>,
    pub all_open_files: Vec<LogFileState>, // TODO: implement management of multiple open files.
    pub current_list: SelectableList<String>,
}

pub enum AggregateEvent {
    Tui(TuiEvent),
    Task(EventResponse),
}

#[derive(Debug, Clone)]
pub struct ModeState {
    pub mode: Mode,
    pub dimensions: Dimensions,
}

#[derive(Debug, Clone)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    // When state is not defined yet
    Undefined,
    // When navigating through the file logs.
    Viewer(),
    // When viewing a line's details.
    Details,
    // When in search/find dialog.
    Find(TextInputCursor, ViewType),
    //
    GoToLine(TextInputCursor, ViewType),
    // When in file import dialog.
    FileImport(TextInputCursor, ViewType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewType {
    Command,
    Dialog,
    Window,
}

impl Mode {
    pub fn is_viewer_command_mode(&self) -> bool {
        match self {
            Mode::Find(_, view_type) |
            Mode::GoToLine(_, view_type) |
            Mode::FileImport(_, view_type) => {
                *view_type == ViewType::Command
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileBatch {
    pub upper: usize,
    pub lower: usize,
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
    pub fn new(sender: Sender<AggregateEvent>) -> Self {
        let viewer_mode = Rc::new(RefCell::new(ModeState::new(Mode::Viewer())));
        Self {
            current_mode: viewer_mode.clone(),
            all_modes: vec![viewer_mode.clone()],
            task_scheduler: TaskScheduler::new(sender),
            ready: false,
            should_quit: false,
            scroll_bar_position: ScrollbarState::default(),
            curr_open_file: None,
            all_open_files: Vec::new(),
            current_list: SelectableList::default(),
        }
    }
}

impl ModeState {
    pub fn new(mode: Mode) -> Self {
        Self { mode, dimensions: Dimensions { width: 0, height: 0 } }
    }

    pub fn new_with_dimensions(mode: Mode, dimensions: Dimensions) -> Self {
        Self { mode, dimensions }
    }
}

fn create_undefined_mode() -> ModeState {
    ModeState {
        mode: Mode::Undefined,
        dimensions: Dimensions {
            width: 0,
            height: 0,
        },
    }
}
