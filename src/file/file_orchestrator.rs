use crate::file::file_state::FileState;
use crate::file::mega_open::gen_mega_open;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use std::thread::{JoinHandle, Thread};

#[derive(Debug)]
pub struct FileOrchestrator {
    pub files: Mutex<HashMap<String, FileState>>,
    pub workers: Mutex<Workers>,
}

#[derive(Debug)]
struct Workers {
    count: usize,
    handlers: Vec<JoinHandle<()>>,
}

static CHUNK_BYTE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

impl FileOrchestrator {
    pub fn new() -> Self {
        Self {
            files: Mutex::new(HashMap::new()),
            workers: Mutex::new(Workers {
                count: 0,
                handlers: Vec::new(),
            }),
        }
    }

    pub fn load_file(&mut self, file_location: String) -> bool {
        if is_already_loaded(self, file_location.clone()) {
            return false;
        }

        let file_state = gen_mega_open(file_location.clone(), CHUNK_BYTE_SIZE);
        self.insert_file(file_location, file_state);

        let mut workers = self.workers.lock().expect("Failed to lock workers!");
        let handler = thread::spawn(move || {});
        workers.count = workers.count + 1;
        workers.handlers.push(handler);

        true
    }

    fn insert_file(&mut self, file_location: String, file_state: FileState) {
        self.files.lock().unwrap().insert(file_location, file_state);
    }
}
fn is_already_loaded(file_orchestrator: &FileOrchestrator, file_location: String) -> bool {
    file_orchestrator
        .files
        .lock()
        .unwrap()
        .contains_key(&file_location)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_find_by_line_number_zero() {
        let start_init = Instant::now();
        let mut file_orchestrator = FileOrchestrator::new();
        let end_init = Instant::now();
        println!(
            "file_orchestrator init took: {:?}",
            end_init.duration_since(start_init)
        );

        let start_load_file = Instant::now();
        let result =
            file_orchestrator.load_file("./test-logs/numbers-hundred-milion.log".to_string());
        let end_load_file = Instant::now();
        println!(
            "load_file execution took: {:?}",
            end_load_file.duration_since(start_load_file)
        );
        println!("result: {:?}", result);
        println!("file orchestrator: {:?}", file_orchestrator);
        assert!(result);
    }
}
