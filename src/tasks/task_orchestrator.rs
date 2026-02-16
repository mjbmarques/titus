use crate::tasks::file::open_file;
use crate::tui::event::{EventRequest, EventResponse};
use crate::tui::state::AggregateEvent;
use std::sync::mpsc::Sender;

pub fn orchestrate_task(task: EventRequest, sender: Sender<AggregateEvent>) {
    match task {
        EventRequest::LoadFile(file_path) => {
            sender
                .send(AggregateEvent::Task(EventResponse::FileContent(
                    file_path.clone(),
                    open_file(file_path),
                )))
                .expect("Failed to send FileContent");
        }
    }
}
