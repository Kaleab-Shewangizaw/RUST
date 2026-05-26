mod reader;
mod runner;

use std::io::{self, Write};
 


fn backend_registry_cli() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("can not read the command!");

        let command = command.trim();

        if command.eq_ignore_ascii_case("exit") {
            break;
        }else if command == "?" {
            println!("available commands:\n
>/list                                    -list all servers
>/run                                     -run server
>/next                                    -change to the next server
>/status                                  -check all servers' status
>/get [s_number]                          -get specific server
>/update [s_number] [key] [new value]     -update server");
        }else if command == "list"{
            
            let backends = reader::read("../config.txt");
            for backend in backends {
                println!("{}:{} {} {}", backend.address, backend.port, backend.status.describe(), backend.weight);
            }

        }else if command == "run" {

            println!("server number [empty to run all servers]: ");
            let mut response = String::new();

            io::stdin()
                .read_line(&mut response)
                .expect("can't read the response");

            let response = response.trim();

            if response == "" {
                println!("running all servers!");
                let backends = reader::read("../config.txt");
                runner::runner(backends, None);
            }else {
                println!("running server {}", response);
                let backends = reader::read("../config.txt");
                runner::runner(backends, Some(response.parse::<u32>().expect("not a valid number")));
            }
            
        }else if command == "status" {
            let backends = reader::read("../config.txt");
            let mut x = 1;
            for backend in backends {
                println!("server {} {}", x, backend.status.describe(), );

                x += 1;
            }
        }else if command == "next" {
            println!("switching to the next server!");
            
    }
}




fn main() {
    println!("\nwelcome,");
    println!("manage your servers easily!");
    println!("/exit to exit");
    println!("/?  for help.\n");

    backend_registry_cli();
}
