mod reader;

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
        }else if command == "list"{
            
            reader::read("../config.txt", true);

        }else if command == "?" {
            println!("available commands:\n
>/list                                    -list all servers
>/run                                     -run server
>/next                                    -change to the next server
>/status                                  -check all servers' status
>/get [s_number]                          -get specific server
>/update [s_number] [key] [new value]     -update server");
        }
        else if command == "run" {

            println!("server number [empty to run all servers]: ");
            let mut response = String::new();

            io::stdin()
                .read_line(&mut response)
                .expect("can't read the response");

            let response = response.trim();

            if response == "" {
                println!("running all servers!");
            }else {
                println!("running server {}", response);
            }

            


            
        }
    }
}




fn main() {
    println!("\nwelcome,");
    println!("manage your servers easily!");
    println!("/exit to exit");
    println!("/?  for help.\n");

    backend_registry_cli();
}
