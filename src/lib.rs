pub mod tui;

pub mod keybinds;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tui::{state::State};
use std::{io};
use std::io::Stdout;
use log::debug;
use ratatui::crossterm::event::KeyEvent;
use crate::tui::event::EventHandler;
use crate::tui::event::Event;
use crate::tui::{Tui};

pub fn start_tui() {
    let crossterm_backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(crossterm_backend);
    if terminal.is_err() {
        println!("Failed to create terminal.");
        return;
    }

    let events = EventHandler::new(250);

    let mut state: State = State::new();
    let terminal = terminal.unwrap();
    let mut my_tui = Tui::new(terminal, events);
    my_tui.init();

    while state.should_quit.eq(&false) {
        tui_loop(&mut my_tui, &mut state).expect("TODO: panic message");
    }
    ratatui::restore();

}

fn tui_loop(my_tui: &mut Tui<CrosstermBackend<Stdout>>, state: &mut State) -> Result<(), String> {
    my_tui.draw(state);

    match my_tui.events.next()? {
        Event::Key(event) => {
            debug!("Key event: {:?}", event);
            keybinds::quit::try_key(state, event);
        },
        Event::FocusGained => {
            debug!("Got focus gained event");
        },
        Event::FocusLost => {
            debug!("Got focus lost event");
        },
        Event::Mouse(mouse_event) => {
            debug!("Got mouse event: {:?}", mouse_event);
        },
        Event::Resize(w, h) => {
            debug!("Resize event: width: {}, height: {}", w, h);
        },
        Event::Paste(value) => {
            debug!("Got paste event: {}", value);
        },
    }
    Ok(())
}