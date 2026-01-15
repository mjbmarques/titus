use std::process::exit;
use titus::start_tui;
use log::{info};

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default())
        .expect("failed to initialize logger");

    info!("Starting TUI application...");
    start_tui();
    info!("TUI application has exiting...");
    exit(0);
}
