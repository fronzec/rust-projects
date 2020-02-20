// a single line comment :D
// importing the 'io' library of the standard 'std' library.
use std::io;

// the entry point for our rust program
fn main() {
    println!("Guess the number!");
    println!("Input your guess.");

    // It creates a mutable 'mut' string
    let mut guess  = String::new();

    // Reads the user input
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line :(");

    // Prints the user input
    println!("Your guessed: {}", guess);
}
