// src/config.rs

use crate::types::BackendStatus;  // pulling from our types module
use std::fmt;

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
}

impl ServerConfig {
    pub fn new(host: &str, port: u16, max_connections: u32) -> Self {
        ServerConfig {
            host: host.to_string(),
            port,
            max_connections,
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "Proxy running at {}:{}, max {} connections",
            self.host, self.port, self.max_connections
        )
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ServerConfig [{}:{} | max_connections: {}]",
            self.host, self.port, self.max_connections)
    }
}