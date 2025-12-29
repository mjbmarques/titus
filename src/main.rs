use titus::start_tui;
use log::{info};

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("Starting TUI application...");
    println!("Hello, world!");
    // start_tui();
    info!("TUI application has exited.");
}
