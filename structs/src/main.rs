struct server {
    host: String,
    port: u16,
    isOnline: bool,
}

impl server {
    fn display(&self) {
        println!("{}, {}, {}", self.host, self.port, self.isOnline);
    }

    fn new(host: String, port: u16, isOnline: bool) -> server {
        server {
            host,
            port,
            isOnline,
        }
    }

    fn disable(&mut self) {
        self.isOnline = false;
    }
}

fn main() {
    let mut s1 = server::new(String::from("localhost"), 3131, true);
    s1.display();
    s1.disable();
    s1.display();
}
