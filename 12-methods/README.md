# Methods syntax

Methods are different from functions in that they're defined within the context of a struct(or a enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

```rust
// This annotation allow include functionality to print out debugging information
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height:  u32,
}

// Implementation block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50}
    println!("The area of the rectangle is {} square pixels.", rect1.area())
}
```

To define the function within the context of `Rectangle`, we start an `impl` (implementation) block. We put the `area` function within the `impl` and change the first parameter to be `self` in the signature and everywhere within the body. After that in the main function we can use _method syntax_ to call the `area` method on our `Rectangle` instance.

The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments. In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle` because Rust knowns the type of `self` due to this method's being inside the `impl Rectangle` context.

In this version we don't want to take ownership. The main benefit of using methods instead of functions, in addition to using method syntax and not having to repeat the type of `self` in every method's signature.

In C and C++, two different operators are used for calling methods: you use `.` if you’re calling a method on the object directly and `->` if you’re calling the method on a pointer to the object and need to dereference the pointer first. Rust has a feature called _automatic referencing and dereferencing_. Calling methods is one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so object matches the signature of the method.

lets add a method with more parameters

```rust
// ...

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

```

Associated Functions

Another useful feature of `impl` blocks is that we're allowed to define functions within `impl` blocks that _don't_ take `self` as a parameter. These are called _associated functions_ because they're associated with the struct. They're still functions, not methods, because they don't have an instance of the struct to work with.

Associated functions are often used for constructors that will return a new instance of the struct. For example, we could provide an associated functionn that would have one dimension parameter and use that as both width and height, thus making it easier to create a square `Rectangle` rather than having to specify the same value twice:

```rust
impl Reactangle {
    fn square/(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}
```

To call this associated function, we use the `::` syntax with the struct name; `let sq = Rectangle::square(3);` the `::` syntax is used for both associated functions and namespaces created by modules.

## Multiple `impl` blocks

Each struct is allowed to have multiple `impl` blocks. Multiple `impl` blocks are useful in generic types and traits.
