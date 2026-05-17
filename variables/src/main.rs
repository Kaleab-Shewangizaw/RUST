fn main() {
   let mut server = String::from("localhost:3434");
   let port: u16 = 3434;
   let online_status: bool = true;
   
   fn print_server_info(server: &String, port: u16, online_status: bool) {
    println!("server: {}", server);
    println!("port: {}", port);
   }

   fn is_server_online(online_status:bool) -> bool {
    online_status
   }

   fn hello_message(message: &str) ->  String {
        format!("hello {}", message)
   }

   print_server_info(&server, port, online_status);
    println!("is server online? {}", is_server_online(online_status));
    println!("{}", hello_message("world"));

}
