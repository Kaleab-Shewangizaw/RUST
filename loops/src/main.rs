fn main() {
    // loop {
    //     println!("this is an infinite loop");
    // }
    let mut x = 0;
    loop {
        x += 1;
        println!("{}", x);

        if x >= 100000 {
            break;
        }
    }

    fn while_loop(mut x: u64){
        while x > 0 {
            println!("{}", x);
            x -= 1;
        }
    }

    fn for_loop(mut j: u64){
        for i in 0..=j{
            println!("hhhhhhhhhhhhhhh, {}", i);
        }
    }

    while_loop(x);
    for_loop(x + 10);
}
 