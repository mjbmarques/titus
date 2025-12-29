use std::io;
use ratatui::backend::Backend;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::terminal::{self, EnterAlternateScreen};
use ratatui::Terminal;
use crate::tui::event::EventHandler;
use crate::tui::state::State;

pub mod state;
pub mod event;

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

    pub fn draw(&mut self, state: &State) {

    }
}