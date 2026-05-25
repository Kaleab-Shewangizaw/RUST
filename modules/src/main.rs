// src/main.rs

mod types;   // load src/types.rs
mod config;  // load src/config.rs
mod logger;  // load src/logger.rs

use config::ServerConfig;
use types::BackendStatus;

fn main() {
    // Build config
    let cfg = ServerConfig::new("127.0.0.1", 8080, 100);

    // Display trait (from config.rs)
    println!("{}", cfg);

    // describe method
    println!("{}", cfg.describe());

    // Log it
    logger::append_log(&cfg.describe());

    // Backend statuses
    let backends = vec![
        BackendStatus::Healthy,
        BackendStatus::Degraded(820),
        BackendStatus::Down(String::from("port 9001 refused")),
        BackendStatus::Unknown,
    ];

    for status in &backends {
        logger::log_status("backend", status);
    }
}