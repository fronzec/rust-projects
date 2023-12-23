use std::io::{self, Read, stdout};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    // Read input from stdin
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        // Note: If we read a character q then exit the program
        if 'q' == c {
            break;
        }
    }

}
