# Rust variables

By default all variables in rust are immutable, e.g. next code will not compile because we try to re assign a variable marked as immutable.

```rust
fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}
```

---

Immutability on variables are optional and we can make a variable mutable:

```rust
fn main() {
    let mut x = 5; // Using 'mut' to make a variable mutable.
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}

```

---

Differences between variables and constants, bot are immutable but constants should be declared with `const` keyword, and require declare the type, it can be declared in any scope, e.g. in global scope. Other difference is, a constant may be set only to a constant expression, not the result of a function call or any other value computed at runtime. Rust's naming convention for constants is to use all uppercase with underscore between words, and underscore can be inserted in numeric literals to improve readability.

```rust
    const MAX_POINTS: u32 = 100_000;
```

---

**Shadowing** : You can declare a variable with the same name as a previous variable, and the new variable shadows the previous variable. Rustaceans says that the first variable is **shadowed** by the second. This means the second variable's value is what appears when the variable is used. To shadow a variable we must use the same variable name and repating the use of the `let` keyword

```rust
    let z = 5;
    let z = z + 1; // Shadows the previous value, z = 6
    let z = z * 2; // Shadows the previous value, z = 12
    println!("The value of z is {}", z); // z = 12
```

Shadowing is different from making a variable as `mut`, we'll get a compile-time error if we try to reassign the variable without using the `let` keyword, using this keyword we can perform a few transformations on a variable but have the resulted variable it going to be immutable after those transformations.

Shadowing effectively creates a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, but we really want to store that input as a number:

```rust
    let spaces = "  ";
    let spaces = spaces.len();

```

However if we try to use `mut` for this,we'll get a compile-time error

```rust
    len mut spaces = "";
    spaces = spaces.len();
```
