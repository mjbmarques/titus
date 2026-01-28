use crate::tui::state::State;
use crate::tui::tui_event::TuiEventHandler;
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;

pub mod colors;
pub mod event;
pub mod file_io;
pub mod state;
pub mod task_adapter;
pub mod tui_event;
mod ui;
pub mod widgets;

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: TuiEventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: TuiEventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn init(&mut self) -> Result<(), ()> {
        terminal::enable_raw_mode().unwrap();
        ratatui::crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)
            .unwrap();
        self.terminal.hide_cursor().unwrap();
        self.terminal.clear().unwrap();
        Ok(())
    }

    pub fn draw(&mut self, state: &mut State) {
        match self.terminal.draw(|frame| ui::render(state, frame)) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error drawing terminal: {}", e);
            }
        }
    }

    pub fn exit(&mut self) {
        terminal::disable_raw_mode().expect("failed to disable raw mode");
        ratatui::crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
            .expect("failed to leave alternate screen");
        self.terminal.show_cursor().expect("failed to show cursor");
        self.events.stop();
    }
}
