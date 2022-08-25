# Control flow

## If condition

```rust
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
```

Rust will not automatically try to convert non-Booleans types to a Boolean. You must explicit and always provide a Boolean condition.

## ff-else

you can handle multiple conditions by combining `if` and `else` and `else if`

```rust
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 and 2");
    }

```

Because `if` is an expression we can use them in a let statement, like this:

```rust
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    // Or like this
    let number = if condition { 5 } else { 6 };
```

## Loops

We can use `loop` keyword to create an infinite loop on a block of code or until you explicitly tell it to stop.

```rust
    loop {
        // code statements
    }
```

Also you can return values from a loop and save it on a variable

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

## Conditional loops with `while`

you can use `while` to create conditional loops, to eliminate loop, if-else, break, like this:

```rust
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
```

## Looping through a collection with `while` and `for`

You can use while constructor to loop over a collection, like this:

```rust
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("The value is {}!", a[index]);
        index += 1;
    }
```

But above form has some problems, we can access to an invalid index accidentally and is slow.

A better approach is use a `for` loop and execute some code for each item in the collection. like this:

```rust
    let a = [10,20,30,40,50];;
    for item in a.iter() {
        println!("The value is : {}", item);
    }
```

Also you can use a `Range`, type provided by the standard library that generates all numbers in sequence starting from one number and ending beforea another, you can iterate in a countdown mode :D, like this using the `rev()` function to reverse the range.

```rust
    for number in (1..3).rev() {
        println!("The value is {}", number);
    }
```
