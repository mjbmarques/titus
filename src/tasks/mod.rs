use crate::tui::event::EventRequest;
use crate::tui::state::AggregateEvent;
use std::sync::mpsc::Sender;

mod file;
mod task_orchestrator;

pub fn run_task(task: EventRequest, sender: Sender<AggregateEvent>) {
    task_orchestrator::orchestrate_task(task, sender);
}
