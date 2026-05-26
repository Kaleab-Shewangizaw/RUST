use std::io;

fn main() {
    // 1. Hardcoded target number for this example
    let secret_number = 7; 
    println!("Welcome to the Guessing Game!");
    println!("Type 'exit' at any time to quit.\n");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 2. Clean the input and check for exit first
        let guess = guess.trim();
        if guess.eq_ignore_ascii_case("exit") {
            println!("Thanks for playing! Goodbye.");
            break;
        }

        // 3. Handle empty input, letters, or numbers
        let guess_num: i32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.is_empty() {
                    println!("You didn't type anything! Try again.\n");
                } else {
                    println!("'{}' is not a valid number. Try again.\n", guess);
                }
                continue; // Skip the rest of the loop and ask again
            }
        };

        // 4. Check the guess
        if guess_num == secret_number {
            println!("🎉 You got it right!");
            
            // 5. Ask to play again
            if !ask_to_play_again() {
                break;
            }
        } else if guess_num < secret_number {
            println!("Too low! Try again.\n");
        } else {
            println!("Too high! Try again.\n");
        }
    }
}

// Helper function to handle the replay prompt
fn ask_to_play_again() -> bool {
    loop {
        println!("Do you want to play a new game? (yes/no):");
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        let response = response.trim().to_lowercase();
        if response == "yes" || response == "y" {
            println!("\nStarting a new round!");
            return true;
        } else if response == "no" || response == "n" {
            println!("Goodbye!");
            return false;
        } else {
            println!("Invalid response. Please type 'yes' or 'no'.");
        }
    }
}
