# Functions

The most important function in rust is `main`, the entry point of many programs. You can create new functions using `fn` keyword. Rust uses **snake** case as convention for funtions and variable names, each word are in lowercase and underscores separate words e.g. `fn my_function(){....}`

A function can have **parameters** which conform the function signature. You can provide values for those parameters these values are called **arguments**.

In function signatures you must declare the type of each parameter. To allow a function have more than one parameter separate each parameter witch commas e.g. `fn funtiona(x:u32, y:u32){...}`

## Statements vs expressions

In rust we must understand the difference between statement and expression. An **statement** are instructions that perform some action and do not return a value. **Expressions** evaluate a resulting value.

**Expressions** do not include semicolons, if you add semicolons to the end of an expression you turn into a statement, which will then not return a value.

## Function with return values

Functions can return values, we declare their type after an arrow `->`. In rust the return value is synonymous with the value of the final expression of the block of the body of a function. You can return early from a function by using `return` keyword and specifying a value, but most functions return the last expression implicitly. like this:

```rust
    fn five() -> i32 {
        5 // remember an expression don't have semicolon
    }
    fn main() {
        let x = five();
        println!("The value of x is {}", x);
    }
```
