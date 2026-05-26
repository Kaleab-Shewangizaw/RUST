use crate::reader::Backend;
use crate::logger::append_log;


pub fn next(backends: Vec<Backend>){
      let mut index = 0;
    for _ in 0..20 {
        let current_item = &backends[index];
        append_log(&format!("Routing to: {}", current_item.address));
        if current_item.status.describe() == "Healthy" {
            println!("{}:{} is healthy, routing to it.", current_item.address, current_item.port);
            append_log(&format!("{}:{} is healthy, routing to it.", current_item.address, current_item.port));
            break;
        } else if current_item.status.describe().starts_with("slow") {
            println!("{}:{} is slow, routing to it.", current_item.address, current_item.port);
            append_log(&format!("{}:{} is slow, routing to it.", current_item.address, current_item.port));
            break;
        } else {
            println!("{}:{} is unhealthy, skipping it.", current_item.address, current_item.port);
            append_log(&format!("{}:{} is unhealthy, skipping it.", current_item.address, current_item.port));
        }
        
        index = (index + 1) % backends.len();
    }
}
