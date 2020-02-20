fn main() {
    /// Invalid compilation
    let x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Valid mutable
    let mut y = 5;
    println!("The value of y is {}", y);
    y = 6;
    println!("The value of y is {}", y);

    // Shadowed variable
    let z = 5;
    let z = z + 1; // Shadows the previous value, 1
    let z = z * 2; // Shadows the previous value, 2
    println!("The value of z is {}", z);

    // Shadowing allow change the value type and use the same variable name
    let spaces =  "  ";
    let spaces = spaces.len();

    // On a mutable variable we can't change the variable name
    let mut spaces = "  ";
    spaces = spaces.len(); // <= compile-time error!
}
}
