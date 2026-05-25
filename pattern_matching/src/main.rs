fn main(){

    fn describe_method(method: &str) -> &str {
        match method {
            "GET" => "for reading data",
            "POST" => "for creating data",
            "PATCH" => "for updating data",
            "PUT" => "for replacing data",
            "DELETE" => "frl deleting data",
            _ => "not a method" 
        }
    }

    let methods = ["GET", "POST", "PATCH", "PUT", "DELETE", "BALA"];

    for method in &methods{
        println!("{} is {}", method, describe_method(method));
    }
}