fn main() {
    hello_world();
    another_function(1);
    let x = five();
    println!("The value of x is {}", x);
    let y = plus_one(6);
    println!("The value of y is {}", y);
}

fn hello_world() {
    println!("Hello world");
}

fn another_function(x:i32) {
    println!("The value of x is {}", x);
}

fn five () -> i32 {
    5 // remember an expression do not have semicolons
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
