#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: u32,
}

fn main() {
    let config = ServerConfig {
        host: String::from("127.0.0.1"),
        port: 8080,
        max_connections: 100,
    };

    println!("{:?}", config);
    // ServerConfig { host: "127.0.0.1", port: 8080, max_connections: 100 }
    println!("Server is running at {}:{} with max {} connections", config.host, config.port, config.max_connections);
    
    
    println!("{:#?}", config); // pretty-printed version
}