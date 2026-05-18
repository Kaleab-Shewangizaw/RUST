use std::fs;

struct ServerConfig {
    host: String,
    port: u16,
    workers: u8
}

fn read_file(path: &str) -> Result<String, String> {

    let content = fs::read_to_string(&path);

    match content {
        Ok(value) => Ok(value),
        Err(_) => Err(String::from("we couldn't find the file!"))
    }
    
}



fn main() {

    let path = "./config.txt";

    let result = read_file(&path);

    
    match result {
        Ok(value) => println!("here {}", value),
        Err(error) => println!("error {}", error)
    }
  
}
