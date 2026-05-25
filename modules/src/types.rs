// src/types.rs

#[derive(Debug)]
pub enum BackendStatus {
    Healthy,
    Degraded(u32),
    Down(String),
    Unknown,
}

impl BackendStatus {
    pub fn describe(&self) -> String {
        match self {
            BackendStatus::Healthy      => String::from("Backend healthy — routing traffic"),
            BackendStatus::Degraded(ms) => format!("Backend slow — {}ms response time", ms),
            BackendStatus::Down(reason) => format!("Backend offline: {}", reason),
            BackendStatus::Unknown      => String::from("Backend status unknown"),
        }
    }
}