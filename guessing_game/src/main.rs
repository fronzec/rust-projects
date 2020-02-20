// This is a single line comment :D
// importing the 'io' library of the standard 'std' library.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// the entry point for our rust program
fn main() {
    println!("======== Guess the number! =========\n");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("\nInput your guess.");
        // It creates a mutable 'mut' string
        let mut guess = String::new();

        // Reads the user input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line :(");

        // Handle invalid input, this pass to next loop iteration on invalid input
        let guess: u32 = match guess.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        // Prints the user input
        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("To big!"),
        }
    }
}
