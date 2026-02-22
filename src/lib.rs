mod file;
mod keybinds;
mod tasks;
mod tui;

use crate::tui::event::{EventRequest, EventResponse};
use crate::tui::state::{AggregateEvent, FileBatch, LogFileState};
use crate::tui::task_adapter::{TaskAdapter, handle_file_content};
use crate::tui::tui_event::TuiEvent;
use crate::tui::tui_event::TuiEventHandler;
use crate::tui::widgets::list::SelectableList;
use crate::tui::{Tui, file_io, task_adapter};
use log::{debug, error, info};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::KeyEvent;
use std::io;
use std::io::Stdout;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use tui::state::State;

pub fn start_tui(starting_file: Option<String>) {
    let crossterm_backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(crossterm_backend);
    if terminal.is_err() {
        println!("Failed to create terminal.");
        return;
    }
    let (sender, receiver) = mpsc::channel();

    let events = TuiEventHandler::new(sender.clone(), 250);

    let mut task_adapter = TaskAdapter::new();
    let mut state: State = State::new(sender);
    let terminal = terminal.unwrap();
    let mut my_tui = Tui::new(terminal, events);
    my_tui.init().expect("Failed to initialize terminal");

    // Trigger event to load initial file if provided.
    if starting_file.is_some() {
        state
            .task_scheduler
            .trigger_task(EventRequest::LoadFile(starting_file.unwrap()));
    }

    while state.should_quit.eq(&false) {
        tui_loop(&receiver, &mut my_tui, &mut state, &mut task_adapter).expect("tui_loop failed");
    }
    ratatui::restore();
}

fn tui_loop(
    receiver: &Receiver<AggregateEvent>,
    my_tui: &mut Tui<CrosstermBackend<Stdout>>,
    state: &mut State,
    task_adapter: &mut TaskAdapter,
) -> Result<(), String> {
    my_tui.draw(state);

    match receiver.recv() {
        Ok(event) => match event {
            AggregateEvent::Tui(tui_event) => {
                handle_tui_events(tui_event, state).expect("handle_tui_events failed");
            }
            AggregateEvent::Task(task_event) => {
                handle_task_events(task_event, task_adapter, state)
                    .expect("handle_task_events failed");
            }
        },
        Err(_) => {
            error!("Error receiving event from channel.");
            return Err("Error receiving event from channel.".to_string());
        }
    }
    Ok(())
}

fn handle_task_events(
    task_event: EventResponse,
    task_adapter: &mut TaskAdapter,
    state: &mut State,
) -> Result<(), String> {
    match task_event {
        EventResponse::FileContent(file_location, content) => {
            task_adapter.file_content(state, file_location, content);
        }
        EventResponse::Error(err) => {
            debug!("EventResponse::Error received: {:?}", err);
        }
        EventResponse::FileSearchContent(_, _) => {
            debug!("EventResponse::FileSearchContent received");
        }
    };
    Ok(())
}

fn handle_tui_events(tui_event: TuiEvent, state: &mut State) -> Result<(), String> {
    match tui_event {
        TuiEvent::Key(event) => {
            debug!("Key event: {:?}", event);
            keybinds::try_keybinds(state, event);
        }
        TuiEvent::FocusGained => {
            debug!("Got focus gained event");
        }
        TuiEvent::FocusLost => {
            debug!("Got focus lost event");
        }
        TuiEvent::Mouse(mouse_event) => {
            debug!("Got mouse event: {:?}", mouse_event);
        }
        TuiEvent::Resize(w, h) => {
            debug!("Resize event: width: {}, height: {}", w, h);
        }
        TuiEvent::Paste(value) => {
            debug!("Got paste event: {}", value);
        }
    }
    Ok(())
}
