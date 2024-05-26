use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die( e: std::io::Error) {
    panic!("{}", e)
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    // Read input from stdin
    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?\r}", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                // On Ctrl + q ends program
                Key::Ctrl('q') => break,
                _ => println!("{:?\r}", key)
            },
            Err(err) => die(err)
        }

    }

}
