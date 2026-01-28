use crate::tui::event::{EventRequest, EventResponse};
use std::sync::mpsc::Sender;
use crate::tui::state::AggregateEvent;

mod file;
mod task_orchestrator;

pub fn run_task(task: EventRequest, sender: Sender<AggregateEvent>) {
    task_orchestrator::orchestrate_task(task, sender);
}
