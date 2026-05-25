fn main() {

    fn check_status(status: Option<&str>) {
        if let Some(s) = status {
            println!("status is {}", s);
        } else {
            println!("status not found");
        }
    }

    enum Status {
        Healthy,
        Dragged(u32),
        Down(String),
        Unknown,
    }

    struct Server {
        name: String,
        status: Status,
    }

    impl Server {
        fn new(name: String, status: Status) -> Self {
            Server { name, status }
        }
    }

    let server1 = Server::new(
        String::from("server1"),
        Status::Healthy,
    );

    fn report_status(server: &Server) {

        match &server.status {

            Status::Healthy => {
                println!("the server is healthy");
            }

            Status::Dragged(sec) if *sec > 1000 => {
                println!("the server is taking too long to respond");
            }

            Status::Dragged(sec) => {

                if matches!(server.status, Status::Healthy) {

                    println!(
                        "the server is healthy but it's taking {} seconds to respond",
                        sec
                    );

                } else {

                    println!(
                        "the server is dragged but it's not healthy"
                    );
                }
            }

            Status::Down(reason) => {
                println!(
                    "the server is down because {}",
                    reason
                );
            }

            Status::Unknown => {
                println!("the server status is unknown");
            }
        }
    }

    report_status(&server1);

    check_status(Some("Good"));
    check_status(None);
}