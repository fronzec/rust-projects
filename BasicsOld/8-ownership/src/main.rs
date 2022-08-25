fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    println!("x = {}", x);

    let s3 = gives_ownership(); // Get ownership from a function
    let s4 = String::from("some");
    let s5 = takes_and_give_back(s4);

    // ==========================================
    // Return values and Scope
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // transfer to compute something
    println!("The length of '{}' is {}.", s2, len); // gives back ownership and additional data using a tuple

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens. Then s3 is dropped, s4 is invalid and nothing happen with them, s5 it goes out of scope and is dropped

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {
    let some_string = String::of("hello");
    some_string
} // some_string is moved


fn takes_and_give_back(s_string:String) -> String { //s_string come to scope
    // do stuff with s_string
    s_string
} // s_string us moved  



fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
