use std::fs;


fn main() {
    let contents = fs::read_to_string("../projects/config.txt");
    match contents {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", String::from("error finding the contnetttttttt"))
    }
}
