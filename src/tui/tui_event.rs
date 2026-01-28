use log::{debug, info};
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};
use std::{sync, thread};
use crate::tui::state::AggregateEvent;

#[derive(Debug)]
pub enum TuiEvent {
    // Key Event
    Key(KeyEvent),
    FocusGained,
    FocusLost,
    Mouse(MouseEvent),
    Resize(u16, u16),
    Paste(String),
}

#[derive(Debug)]
pub struct TuiEventHandler {
    pub sender: Sender<AggregateEvent>,
    pub handler: thread::JoinHandle<()>,
    // Is the event handler active / listening for events.
    pub active: Arc<AtomicBool>,
}

impl TuiEventHandler {
    pub fn new(sender: Sender<AggregateEvent>, tick_rate: u64) -> Self {
        let active = Arc::new(AtomicBool::new(true));
        let handler = setup_thread(sender.clone(), tick_rate, active.clone());
        Self {
            sender,
            handler,
            active,
        }
    }

    pub fn stop(&mut self) {
        self.active.store(false, Ordering::Relaxed);
    }
}

fn setup_thread(
    sender: Sender<AggregateEvent>,
    tick_rate: u64,
    is_active: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    let tick_duration = Duration::from_millis(tick_rate);

    thread::spawn(move || {
        let mut last_tick = Instant::now();

        while is_active.load(Ordering::Relaxed) {
            let timeout = calculate_timeout(&tick_duration, &last_tick);
            handle_tui_events(&sender, timeout);

            if last_tick.elapsed().ge(&tick_duration) {
                last_tick = Instant::now();
            }
        }
    })
}

fn handle_tui_events(sender: &Sender<AggregateEvent>, timeout: Duration) {
    if event::poll(timeout).expect("event polling failed") {
        match event::read().expect("event reading failed") {
            CrosstermEvent::FocusGained => sender.send(AggregateEvent::Tui(TuiEvent::FocusGained)),
            CrosstermEvent::FocusLost => sender.send(AggregateEvent::Tui(TuiEvent::FocusLost)),
            CrosstermEvent::Key(key) => sender.send(AggregateEvent::Tui(TuiEvent::Key(key))),
            CrosstermEvent::Mouse(mouse) => sender.send(AggregateEvent::Tui(TuiEvent::Mouse(mouse))),
            CrosstermEvent::Resize(w, h) => sender.send(AggregateEvent::Tui(TuiEvent::Resize(w, h))),
            CrosstermEvent::Paste(value) => sender.send(AggregateEvent::Tui(TuiEvent::Paste(value))),
        }
        .expect("failed to send event through channel");
    }
}

fn calculate_timeout(tick_rate: &Duration, last_tick: &Instant) -> Duration {
    tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or(*tick_rate)
}
