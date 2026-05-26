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
        }
    }
}




fn main() {
    println!("\nwelcome,");
    println!("manage your servers easily!\n");

    backend_registry_cli();
}
