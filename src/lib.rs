pub mod tui;

pub mod keybinds;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tui::{state::State};
use std::{io};
use std::io::Stdout;
use log::debug;
use ratatui::crossterm::event::KeyEvent;
use crate::tui::tui_event::TuiEventHandler;
use crate::tui::tui_event::TuiEvent;
use crate::tui::{file_io, Tui};
use crate::tui::state::{FileBatch, LogFileState};
use crate::tui::widgets::list::SelectableList;

pub fn start_tui() {
    let crossterm_backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(crossterm_backend);
    if terminal.is_err() {
        println!("Failed to create terminal.");
        return;
    }

    let events = TuiEventHandler::new(250);

    let mut state: State = State::new();
    let terminal = terminal.unwrap();
    let mut my_tui = Tui::new(terminal, events);
    my_tui.init();

    // render inside the outside block, in this case, the content itself.
    let file_location = String::from("./log/titus-2025-12-30_15-10-47.log");
    let hardcoded_content = get_test_strings(file_location);
    // TODO: Ultra hardcoded for testing purposes only.
    state.curr_open_file = Some(LogFileState{
        id: 1,
        file_name: String::from("./log/titus-2025-12-30_15-10-47.log"),
        file_path: String::from("./log/titus-2025-12-30_15-10-47.log"),
        file_lines: hardcoded_content.len(),
        file_loaded_lines: FileBatch{ upper: hardcoded_content.len().saturating_sub(1) , lower: 0 } ,
        loaded_lines: SelectableList::with_items(hardcoded_content),
    });
    // TODO: Be careful with this line, must be changed in the future.
    state.current_list = state.curr_open_file.clone().unwrap().loaded_lines;

    while state.should_quit.eq(&false) {
        tui_loop(&mut my_tui, &mut state).expect("TODO: panic message");
    }
    ratatui::restore();
}

// For testing purposes only; to be replaced with real file opening logic, and in the right place
// which is not here at all.
fn get_test_strings(file_location: String) -> Vec<String> {
    let mut lines_vec: Vec<String> = Vec::new();
    if let Ok(lines ) = file_io::read_lines(file_location) {
        for line in lines.map_while(Result::ok) {
            lines_vec.push(line);
        }
    };
    lines_vec
}

fn tui_loop(my_tui: &mut Tui<CrosstermBackend<Stdout>>, state: &mut State) -> Result<(), String> {
    my_tui.draw(state);

    match my_tui.events.next()? {
        TuiEvent::Key(event) => {
            debug!("Key event: {:?}", event);
            keybinds::try_keybinds(state, event);
        },
        TuiEvent::FocusGained => {
            debug!("Got focus gained event");
        },
        TuiEvent::FocusLost => {
            debug!("Got focus lost event");
        },
        TuiEvent::Mouse(mouse_event) => {
            debug!("Got mouse event: {:?}", mouse_event);
        },
        TuiEvent::Resize(w, h) => {
            debug!("Resize event: width: {}, height: {}", w, h);
        },
        TuiEvent::Paste(value) => {
            debug!("Got paste event: {}", value);
        },
    }
    Ok(())
}