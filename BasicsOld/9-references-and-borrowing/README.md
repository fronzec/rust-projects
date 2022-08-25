# References and borrowing

## Borrowing

Here is how you would define and use a _calculate_length_ function that has a reference to an object as a parameter instead of taking ownership of the value:

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
}

fn calculate_legth(s:&String) -> usize { // s is a reference to a String
    s.len()
}// Here, s goes out of scope, But because it does not have ownership of what is refers to, nothing happens.
```

Note we pass _&s1_ into _calculate_length_ and we take a _&String_ rather than _String_.

These ampersands are _references_ , and they allow you to refer to some value without taking ownership of it.

```console
s                          s1               heap
--------------       --------------     -----------------
|name|value  |       |name|value  |     |index  |value  |
|----|-------|       |------------|     -----------------
|ptr |   ----|--->   |ptr |-------|---> |0      |  H    |
                                        |1      |  E    |
                                        |2      |  L    |
                                        |3      |  L    |
                                        |4      |  O    |
```

**NOTE**: The opposite of referencing by using `&` is _dereferencing_, which is accomplished with the dereference operator `*`.

The `&s1` syntax lets us create a reference that _refers_ to the value os `s1` but doesn't own it. Because it does not own it, the value it points to will not be dropped when the reference goes out of scope. The scope in which the variable `s` is valid is the same as any function parameter's scope.

**We call having references as functions parameters as _borrowing_**

Just as variables are **immutable by default, so are references**. We're not allowed to modify something we have a reference to.

## Mutable References

We can create mutable references like this:

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First we had to change `s` to be `mut`. Then we had to create a mutable reference with `&mut s` and accept a mutable reference with `some_string: &mut String`.

But mutable references have one only big restriction: **you can have only one mutable reference to a particular piece of data on a particular scope**, e.g. next code will fail:

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
// do stuff
```

The error will be:

```console
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

```

This restriction allows for mutation but in a very controlled fashion. The benefit of having is that Rust can prevent data races at compile time. A _data race_ is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix. Rust prevents this because it won't even compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.
let r2 = &mut s;
```

A similar rule exist for combining mutable and immutable references. This code results in an error:

```rust
let mut s = String::from("hello");
let r1 = &s; // No problem
let r2 = &s; // No problem
let r3 = &mut s; // BIG PROBLEM
// do stuff
```

Here's the error

```console
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

```

We cannot have a mutable reference while we have an immutable one. Users of an immutable reference don't expect the values to suddenly change out from under them!. However, multiple immutable references are okay because no one woho just is reading the data has the ability to affect anyone else's reading on the data.

Note that a **reference's scope** **starts** from **where it is introduced** and continues through **te last time that reference is used**.

```rust
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} {}", r1, r2);
// r1 and r2 are no longer used after this point, their scopes ends here

let r3 =  &mut s; // no problem, we can create a mutable reference because don't overlap with the scope of r1 and r2
// do stuff with r3

```

## Dangling references

**dangling reference**: a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory.

In Rust the compiler guarantees that references will never be dangling references: If you have a reference to some data, the compiler will ensure that the data will no go out of scope before the reference to the data does.

Like this:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // We return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away DANGER !!!
```

Here's the error:

```console
error[E0106]: missing lifetime specifier
 --> main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime

```

When the code of `dangle` is finished, `s` will be deallocated. But we tried to return a reference to ir. THat means this reference would be pointing to an invalid `String`. The solution here is to return the `String` directly to move out the ownership.

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership is moved out
} // Nothing is deallocated
```

## The rules of references

- At any given time, you can have _either_ one mutable reference _or_ any number of immutable reference.
- References must always be valid.
