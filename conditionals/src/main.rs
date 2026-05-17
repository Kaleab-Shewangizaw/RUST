fn main() {
    let is_admin: bool = false;

    if !is_admin{
        println!("you are not an admin");
    }

    fn can_vote(age: u8) -> bool {
        if age >= 18 {
            true
        } else {
            false
        }
    }

    let age: u8 = 20;
    println!("can you vote? {}", can_vote(age));


    let user_name = "john_doe";
    match user_name {
        "admin" => println!("welcome admin"),
        "john_doe" => println!("welcome john_doe"),
        _ => println!("welcome guest")
    }

    let response_code = 404;

    match response_code{
        200..=299 => println!("success"),
        300..=399 => println!("redirection"),
        400..=499 => println!("client error"),
        500..=599 => println!("server error"),
        _ => println!("unknown response code")
    }
}
