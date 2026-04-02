use crate::file::file_state::FileState;
use log::debug;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

mod file_orchestrator;
mod file_state;
pub(crate) mod find;
mod mega_open;

static CHUNK_BYTE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
static FILES: LazyLock<Mutex<HashMap<String, FileState>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
pub fn load_file(file_location: String) -> bool {
    debug!("Loading file: {}. Requesting lock().", file_location);
    let mut files = FILES.lock().expect("Failed to acquire lock on FILES");
    debug!("Acquired lock on FILES. Loading file: {}", file_location);
    if files.contains_key(&file_location) {
        debug!("Requested to open file already loaded: {}", file_location);
        return false;
    }

    let file_state = mega_open::gen_mega_open(file_location.clone(), CHUNK_BYTE_SIZE);

    true
}
