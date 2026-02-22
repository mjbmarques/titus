use crate::tasks::run_task;
use crate::tui::state::AggregateEvent;
use std::sync::mpsc::Sender;
use std::thread;

#[derive(Debug)]
pub enum EventResponse {
    FileContent(String, Vec<String>),
    FileSearchContent(Vec<String>, usize),
    Error(EventError),
}

#[derive(Debug)]
pub enum EventError {
    Unknown(String),
    FileNotFound(String),
}

#[derive(Debug)]
pub enum EventRequest {
    LoadFile(String),
    FindInFile(String, usize, usize),
}

#[derive(Debug)]
pub struct TaskScheduler {
    // sender to send event responses back to the main thread.
    pub sender: Sender<AggregateEvent>,
    // TODO: handle handlers cleanup/deletion when they finish.
    pub handlers: Vec<thread::JoinHandle<()>>,
    // Is the event handler active / listening for events.
    pub active: bool,
}

impl TaskScheduler {
    pub fn new(sender: Sender<AggregateEvent>) -> Self {
        let active = true;
        Self {
            sender,
            handlers: Vec::new(),
            active,
        }
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn trigger_task(&mut self, task_type: EventRequest) {
        let sender_clone = self.sender.clone();
        let handler = thread::spawn(move || {
            run_task(task_type, sender_clone);
        });
        self.handlers.push(handler);
    }
}
