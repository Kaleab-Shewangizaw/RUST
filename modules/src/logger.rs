// src/logger.rs

use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn append_log(message: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("proxy.log")
        .unwrap();

    let entry = format!("[{}] {}", timestamp, message);

    writeln!(file, "{}", entry).unwrap();
    println!("{}", entry);  // also print to terminal
}

pub fn log_status(label: &str, status: &crate::types::BackendStatus) {
    append_log(&format!("{}: {}", label, status.describe()));
}