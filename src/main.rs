use log::info;
use std::env;
use std::process::exit;
use titus::start_tui;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default())
        .expect("failed to initialize logger");

    info!("Starting TUI application...");
    let args: Vec<String> = env::args().collect();
    info!("Args: {:?}", args);
    let file_to_open = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };
    start_tui(file_to_open);
    info!("TUI application has exiting...");
    exit(0);
}
