pub enum BackendStatus {
    Healthy,
    Dragged(u32),
    Down(String),
    Unknown,
}

pub enum Function {
    Running,
    Stopped(String)
}



impl BackendStatus {
    pub fn describe(&self) -> String {
        match self {
            BackendStatus::Healthy => String::from("Healthy"),
            BackendStatus::Dragged(ms) => format!("slow{}", ms),
            BackendStatus::Down(reason) => format!("offline: {}", reason),
            BackendStatus::Unknown => String::from("unknown"),
        }
    }
}

impl Function {
    pub fn describe(&self) -> String {
        match self {
            Function::Running =>  String::from("Running"),
            Function::Stopped(reason)  => format!("err! server {}", reason)
        }
    }
}