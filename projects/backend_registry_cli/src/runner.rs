
use crate::logger::append_log;
use crate::reader::Backend;


pub fn runner(backends: Vec<Backend>, server_number: Option<u32>) {
    if server_number.is_none(){
        let mut x = 1;
    for backend in backends {
        if backend.status.describe() == "Healthy" {
            println!("{} {}:{} {} {} status: {}, started", x, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe());
            append_log(&format!("{} {}:{} {} {} status: {}, started", x, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe()));

            x += 1;
        }else if backend.status.describe().starts_with("slow") {
            println!("{} {}:{} {} {} status: {}, started", x, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe());
            append_log(&format!("{} {}:{} {} {} status: {}, started", x, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe()));
            x += 1;
        }else {
            println!("{} {}:{} {} {} status: {}, can't start", x, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe());
            append_log(&format!("{} {}:{} {} {} status: {}, can't start", x, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe()));
            x += 1;
        }
    }
    }else {
        let server_number = server_number.unwrap();
        if server_number == 0 || server_number as usize > backends.len() {
            println!("invalid server number!");
            return;
        }
        let backend = &backends[server_number as usize - 1];
        if backend.status.describe() == "Healthy" {
            println!("{} {}:{} {} {} status: {}, started", server_number, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe());
            append_log(&format!("{} {}:{} {} {} status: {}, started", server_number, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe()));
        }else if backend.status.describe().starts_with("slow") {
            println!("{} {}:{} {} {} status: {}, started", server_number, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe());
            append_log(&format!("{} {}:{} {} {} status: {}, started", server_number, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe()));
        }else {
            println!("{} {}:{} {} {} status: {}, can't start", server_number, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe());
            append_log(&format!("{} {}:{} {} {} status: {}, can't start", server_number, backend.address, backend.port, backend.status.describe(), backend.weight, backend.status.describe()));
        }
    }
}