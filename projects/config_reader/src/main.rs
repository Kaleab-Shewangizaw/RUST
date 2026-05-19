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

    let path = "../config.txt";

    let result = read_file(&path);

    
    match result {
        Ok(value) => {
            let mut conf = ServerConfig {
                host: String::new(),
                port: 0,
                workers: 0
            };

            let mut x = 0;
            for line in value.lines() {
                
                let ( _name,  value) = line.split_once("=").unwrap();
                println!("{}", value);
                x += 1;
                if x == 1 {
                    conf.host = String::from(value);
                   
                } else if x == 2 {
                    conf.port = value.parse::<u16>().expect("not a valid number");
                   
                } else {
                    conf.workers = value.parse::<u8>().expect("not a valid number")
                }
            }

            println!("server is running at {}:{} with {} workers", conf.host, conf.port, conf.workers)
        },
        Err(error) => println!("error {}", error)
    }
  
}
