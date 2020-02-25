# The slice type

Another data type that does not have ownership is the _slice_.
Slice let you reference a contiguous sequence of elements in a collection rather than the whole collection.

Write a function that takes a string and returns the first word it finds in that string. If the function doesn't find a spce in the string, return the whole string.

```rust
fn first_word(s: &string) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

We'll convert our `String` element by element and check whether a value is a space, we'll convert our `String` to an array of bytes using the `as_bytes` method. `iter` is a method that returns each element in a collection and that `enumerate` wraps the result of `iter` and returns each element as a part of a tuple instead, the first element of the tuple returned from `enumerate` is the index, and the second element is a reference to the element. We can use patterns to destructure that tuple.

Inside the for loop, we search fo the byte that represents the space by using the byte literal syntax. If we find a space we return the position. Otherwise we return the length of the String by using `s.len()`.

We're returning a `usize` on its own, but it's only a meaningful number in the context of `&String`. There's no guarantee that it will still be valid in the future. Consider next:

```rust
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear()// Empties the String, making it equal to ""
    // world still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is totally invalid!
}
```

Because `word` isn't connected to the state of `s`, we could have a bug changing the content of s and the index in `word` getting out of sync. Luckily Rust has a solution to this problem: **string slices**

## String slices

A string slice ia reference to part of a `String`, and it looks like this:

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

To create a slice we use a range within brackets `[starting_index..ending_index]`, `starting_index` is the first position and is inclusive, and `ending_index` is one more than the last position in the slice (exclusive).

Internally, the slice data structure stores the starting point and the length of the slice.

You can do:

```rust
// Drop the value before two periods to start at the first index(zero)
let s = String::from("hello");
let slice = &s[0::2];
let slice = &s[..2];

// Drop the trailing number to include the last byte of the String
let s = String::from("hello");
let len = s.len();
let slice = &s[3..len];
let slice = &s[3..]

// Drop both values to take a slice of the entire string
let s = String::from("hello");
let len = s.len();
let slice = &s[0..len];
let slice = &s[..]

```

**NOTE**: String slices range indices must occur at valid UTF-8 character boundaries. Check documentation to for UTF-8 encoded text in Strings.

**The type that signifies "string slice" is written as `&str`**

We can rewrite the function using String slices like this:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return from zero index to actual index
        }
    }

    &s[..]  // return whole string
}
```

Now the compiler will ensure the references into the String remain valid. Now if we try to compile our code `first_word` will throw a compile-time error:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

And the error is:

```console
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

```

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. Rust disallows this, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!

## String literals are slices

```rust
let s = "hello world";
```

The type of `s` here is `&str`: it's a slice pointing to that specific point of the binary. This is also why string literals are immutable.`&str` is an immutable reference.

## String slices as parameters

We can rewrite our function with a string slice parameter:

```rust
fn first_word(s:&str) -> &str {
    // ...
}
```

Now we have a more general and useful API.

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of 'String's
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string literals already,
    // this works too, without the slice sintax!
    let word = first_word(my_string_literal);

}
```

## Other slices

There's a more general slice type, consider this.

```rust
    let a = [1,2,3,4,5];
```

To refer a part of an array, We'd do so like this:

```rust
let slice = &a[1..3];
```

This slice has the type `&[i32]`. The slice also store the reference to he first element and a length.

## Summary

The concepts of ownership, borrowing and slices ensure memory safety in Rust programs at compile time. Rust gives you control over your memory usage like other system programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don't have to write and debug extra code to get this control.
