use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum Event {
    // Key Event
    Key(KeyEvent),
    FocusGained,
    FocusLost,
    Mouse(MouseEvent),
    Resize(u16, u16),
    Paste(String),
}

#[derive(Debug)]
pub struct EventHandler {
    pub sender: Sender<Event>,
    pub receiver: Receiver<Event>,
    pub handler: thread::JoinHandle<()>,
    // Is the event handler active / listening for events.
    pub active: Arc<AtomicBool>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let (sender, receiver) = mpsc::channel();
        let active = Arc::new(AtomicBool::new(true));
        let handler = setup_thread(sender.clone(), tick_rate, active.clone());
        Self {
            sender,
            receiver,
            handler,
            active,
        }
    }

    pub fn stop(&mut self) {
        self.active.store(false, Ordering::Relaxed);
    }

    pub fn next(&self) -> Result<Event, String> {
        match self.receiver.recv() {
            Ok(event) => Ok(event),
            Err(_) => Err(String::from("Failed to receive event")),
        }
    }
}

fn setup_thread(sender: Sender<Event>, tick_rate: u64, is_active: Arc<AtomicBool>) -> thread::JoinHandle<()> {
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

fn handle_tui_events(sender: &Sender<Event>, timeout: Duration) {
    if event::poll(timeout).expect("event polling failed") {
        match event::read().expect("event reading failed") {
            CrosstermEvent::FocusGained => { sender.send(Event::FocusGained).unwrap() },
            CrosstermEvent::FocusLost => { sender.send(Event::FocusLost).unwrap() },
            CrosstermEvent::Key(key) => { sender.send(Event::Key(key)).unwrap() },
            CrosstermEvent::Mouse(mouse) => { sender.send(Event::Mouse(mouse)).unwrap() },
            CrosstermEvent::Resize(w, h) => { sender.send(Event::Resize(w, h)).unwrap(); },
            CrosstermEvent::Paste(value) => { sender.send(Event::Paste(value)).unwrap() },
        }
    }
}

fn calculate_timeout(tick_rate: &Duration, last_tick: &Instant) -> Duration {
    tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or(*tick_rate)
}