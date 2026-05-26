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
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
            reader::Read("../config.txt", true);

        }else if command == "?" {
            println!("available commands:\n
>/list                                    -list all servers
>/run                                     -run all servers
>/next                                    -change to the next server
>/status                                  -check all servers' status
>/get [s_number]                          -get specific server
>/update [s_number] [key] [new value]     -update server");
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
