// a single line comment :D
// importing the 'io' library of the standard 'std' library.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// the entry point for our rust program
fn main() {
    println!("Guess the number!");
    loop {
        println!("Input your guess.");
        let secret_number = rand::thread_rng().gen_range(1, 101);
        // It creates a mutable 'mut' string
        let mut guess = String::new();

        // Reads the user input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line :(");

        let guess: u32 = guess.trim().parse().expect(" Please type a number");

        // Prints the user input
        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Equal => println!("You win!"),
            Ordering::Greater => println!("To big!"),
        }
    }
}
