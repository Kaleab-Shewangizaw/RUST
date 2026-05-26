use std::fs;
mod types;

use types::BackendStatus;
use types::Function;


pub struct Backend {
    pub address: String,
    pub port: u16,
    pub status: BackendStatus,
    pub weight: u32,
    pub function: Function,

}


pub fn get_file(path: &str) -> Result<String, String> {
        let content = fs::read_to_string(&path);

        match content {
            Ok(value) => Ok(value),
            Err(_) => Err(String::from("we couldn't find the file!"))
        }
}

pub fn read(path: &str) -> Vec<Backend> {
    let content = get_file(path);

    let mut backends = Vec::new();

    match content {

        Ok(value) => {
            for line in value.lines() {

                let mut backend1 = Backend {
                address: String::new(),
                port: 0,
                status: BackendStatus::Unknown,
                weight: 0,
                function: Function::Running

            };
                let parts: Vec<&str> = line.split(',').collect();
                let name = parts[0].trim();
                let status = parts[1].trim();
                let weight = parts[2].trim();

                backend1.address = String::from(name).split(":")
                                    .nth(0)
                                    .unwrap()
                                    .to_string();
                backend1.port = name
                                    .split(":")
                                    .nth(1)
                                    .unwrap()
                                    .parse::<u16>()
                                    .expect("not a valid number");
                backend1.status = match status {
                    "healthy" => BackendStatus::Healthy,
                    "dragged" => BackendStatus::Dragged(100),
                    "down" => BackendStatus::Down(String::from("unknown reason")),
                    _ => BackendStatus::Unknown
                };
                backend1.weight = weight.parse::<u32>().expect("not a valid number");
                backends.push(backend1);
            }

        }

        Err(error) => println!("error {}", error)
    }
    backends
}