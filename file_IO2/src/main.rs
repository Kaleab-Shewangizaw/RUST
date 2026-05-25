use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("proxy.log")
        .unwrap();



    writeln!(file, "new log entryyyy").unwrap();


    let data = [1, 2, 3, 4, 5];
    
    // This prints the value at index 3 (which is 4)
    println!("here {}", data[3]); 
    
    // This debug-prints the entire array
    println!("The whole array is: {:?}", data); 
}