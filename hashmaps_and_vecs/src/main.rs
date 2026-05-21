use std::collections::HashMap;


fn main() {
    let mut headers = HashMap::new();
    
    headers.insert(
        String::from("hash1"),
        String::from("hash2");
        String::from("hash3"),
        String::from("hash4")
    );

    if let Some(value) = headers.get("hash1") {
    println!("{}", value);
}

    let mut servers: Vec<String> = Vec::new();

    servers.push(String::from("here is the first content"));

    let first = servers.get(0);

    if let Some(server) = first {
        println!("{}", server);
    }
}