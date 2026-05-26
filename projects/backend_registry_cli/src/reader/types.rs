pub enum BackendStatus {
    Healthy,
    Dragged(u32),
    Down(String),
    Unknown,
}


impl BackendStatus {
    pub fn describe(&self) -> String {
        match self {
            BackendStatus::Healthy => String::from("Healthy"),
            BackendStatus::Dragged(ms) => format!("slowed by {}ms", ms),
            BackendStatus::Down(reason) => format!("offline: {}", reason),
            BackendStatus::Unknown => String::from("unknown"),
        }
    }
}