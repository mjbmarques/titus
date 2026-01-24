use std::io;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use crate::tui::event::EventHandler;
use crate::tui::state::State;

pub mod state;
pub mod event;
mod ui;
pub mod file_io;
pub mod widgets;
pub mod colors;

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn init(&mut self) -> Result<(), ()> {
        terminal::enable_raw_mode().unwrap();
        ratatui::crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
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
        ratatui::crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).expect("failed to leave alternate screen");
        self.terminal.show_cursor().expect("failed to show cursor");
        self.events.stop();
    }
}