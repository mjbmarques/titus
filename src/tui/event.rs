use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::info;

#[derive(Debug)]
pub enum Event<> {
    FileContent,
}

pub struct EventHandler {
    pub sender: Sender<Event>,
    pub receiver: Receiver<Event>,
    pub handler: thread::JoinHandle<()>,
    // Is the event handler active / listening for events.
    pub active: Arc<AtomicBool>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let active = Arc::new(AtomicBool::new(true));
        let handler = setup_thread();
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
        info!("event next called");
        match self.receiver.recv() {
            Ok(event) => {
                // debug!("event received: {:?}", event);
                Ok(event)
            },
            Err(_) => Err(String::from("Failed to receive event")),
        }
    }
}

fn setup_thread() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        // Event handling logic here
    })
}