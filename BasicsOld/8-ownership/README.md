# Ownership on Rust

## The stack and the heap

Both are parts of memory that are available to our code at runtime.
The stack stores values in the order it gets them and removes the values in the opposite order. Referred as _last-in first-out_ or _LIFO_, like a stack of plates, when you add more plates, you put them in the top of the pile and when you need a plate you take one off the top. This both actions are called pushing onto the stack and popping off the stack.

All data stored in the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized, when you put data you request a certain amount of space. The operating system finds an empty spot on the heap that is big enough, marks it as being in use and return a _pointer_. This process is called _allocating on the heap_. **Pushing values to the stack is not considered allocating** because the pointer is a known, fixed size.

Pushing to the stack is faster than allocating on the heap because the OS never has to search for a place to store new data.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. A processor can do its job better if it works on data that's close to other data(as it is on the stack) rather than father away(as it can be on the heap). Allocating a large amount of space on the heap can also take time.

When you code calls a function, the values passed, including potentially pointers to data on the heap and the function's local variables get pushed onto the stack, when the function is over those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that managing heap data is why ownership exists can help explain why it works the way it does.

## Ownership rules

- Each value in Rust has a variable that's called its _owner_.
- There can only be one owner at a time.
- When the owner goes of out scope, the value will be dropped.

## Variable scope

A variable is valid from the point at which it's declared until the end of the current _scope_.

```rust
{                       // s is not valid
    let s = "hello";    // s is valid from this point forward
    // do stuff with s
}                       // this scope is now over, and s is no longer valid
```

## Memory and allocation

String literal are fast and efficient, but this properties only come from the string literal's immutability.

With the _String_ type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, this means:

- The memory must be requested from the operating system at runtime.
- We need a way of returning this memory to the operating system when we're done with out _String_.

In languages with a _Garbage Collector(GC)_ the GC keeps track and cleans up memory that isn't being used anymore, and we don't need to think about it. In the other hand without a GC, it's our responsibility to identify when memory is no longer being used and call code to explicitly return it. If we forget, we'll waste memory, if we do it too early, we'll have an invalid variable, If we do it twice, that's bug too. We need to pair exactly one **allocate** with exactly one **free**.

On Rust: the memory is automatically returned once the variable that owns it goes out of scope, like that.

```rust
{
    let s = String::from("Hello");  // s is valid from this point forward
    // do stuff with s
}                                   // this scope is now over and s is not longer valid
```

When a variable goes out of scope, Rust call the special function called **drop**, Rust calls this function automatically at the closing curly bracket.

- In C++ this pattern of deallocating resources at the end of an item's lifetime is sometimes called _Resource Acquisition Is Initialization (RAII)_.

## Ways Variables and Data Interact

### Move

Multiple variables can interact with the same data in different ways.

```rust
    let x = 5;
    let y = x;
```

Because we are working with simple values with a known fixed size, bind the value _5_ to _x_, then make a copy of the value in _x_ and bind it to _y_.

Now let's a look at the _String_ version:

```rust
    let s1 = String::from("hello");
    let s2 = s1;
```

In this case Rust works in different manner. A string is made up of 3 parts: _a pointer to the memory_, _a length_
and a _capacity_. This group of data is stored on the stack. The content is stored on the heap.

When we assign _s1_ to _s2_, the _String_ data is copied, meaning we copy the pointer, the length and the capacity that are on the stack. We don't copy the data on the heap that the pointer refers to.

![data-copy.](<Screenshot(20).png>)

Because two data pointers pointing to the same location, when s1 and s2 go out of scope, they will both try to free the same location, This is a known as a _double free_ error.

To ensure memory safety there's one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory, Rust consider _s1_ to no longer be valid and , therefore, Rust doesn't need to free anything when _s1_ goes out of scope.

```rust
    let s1 = String::from("hello");
    let s2 = s1;
    print!("{}, world", s1);
```

Previous code will throw next error

```
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

The concept of copying the pointer, length and capacity without copying the data sounds like a shallow copy, But because Rust invalidates the first variable, it's known as a _move_. _s1_ is moved to _s2_.

With only _s2_ valid when we goes out of scope, it alone will free the memory and we're done.

**By design, Rust will never automatically create "deep" copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.**

### Clone

If we do want to deeply copy the heap data of the _String_, we can use a common method called _clone_. Like this:

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
```

Clone is expensive.

## Stack-Only Data: Copy

For this example

```rust
   let x = 5;
   let y = x;
   print!("x = {}, y = {}", x, y);  // both x and y still valid
```

We don't have to call to _clone_ and _x_ is still valid and wasn't moved into _y_, the reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. In other words, there's no difference between deep and shallow copying here, so calling _clone_ wouldn't do make difference from the usual shallow copying.

Rust has a special annotation called the _Copy_ trait, this can be placed on types like integers that are stored on the stack. If a type has the _Copy_ trait, an older variable **is still usable** after assignment. If a type or any of its parts, has implemented the _Drop_ trait Rust won't let us annotate that type with the _Copy_ trait.

What types are _Copy_? As a general rule, any group of simple scalar values can be _Copy_, and nothing that requires allocation or some form of resource is _Copy_, e.g.

- All integers types.
- The Boolean type.
- All the floating-point types.
- The character type.
- Tuples, **if they only contains types that are also _Copy_. For example,_(i32, i32)_ is _Copy_, but _(i32,String)_ is not**.

Check the documentation for additional details.

## Ownership and Functions

The semantics to passing a value to a function are similar to those assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. like this:

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

```

## Return values and Scope

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

The ownership of a variable follows the same pattern every time, Taking ownership and then returning ownership with every function is a bit tedious. It's possible to return multiple values using a tuple, to return ownership and the additional data resulting from the body of our function.

```rust
fn main() {
    let s1 = String::from("hello");
    let (s1, length) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();// return the length of a String
    (s, length)
}
```

But still is to much ceremony and a lot of work for a concept that should be common. Luckily for us **Rust has a feature for this concept, called _references_**
