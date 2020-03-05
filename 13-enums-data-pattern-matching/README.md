# Enums and Pattern Matching

_Enumerations_ also referred as _enums_ allow you to define a type by enumerating its possible _varianst_.

Rust's enums are most similar to _algebraic data types_ in functional languages.

## Defining an enum

Enums are very util when you can _enumerate_ all possible variants, enums values can only be one of its variants. For example if we want to enumerate the two versions of IP addresses we could write an enum like this:

```rust
    enum IpAddrKind {
        V4,
        V6,
    }
```

Now we have a custom data type that we can use elsewhere in our code.

## Enum Values

We can create instance of each of the wo variants of `IpAddrKind` like this:

```rust
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. Both values are of the same type: `IpAddrKind`.

We can define a function that takes any `IpAddrKind`:

```rust
    fn route(ip_kind: IpAddrKind) { }
```

and we can call this function with either variant:

```rust
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
```

We can use enums inside structs like this:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

We also can represent the same concept by putting data directly into each enum variant. This new definition of the `IpAddr` enum says that both `V4` and `V6` variants will have associated `String` values.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

```

We attach data to each variant of the enum directly, so there is no need for an extra struct.

Another advantage, each variant can have different types and amounts of associated data. For example if we want to store `V4` as four `u8` values but still express `V6` addresses as one `String`, enums handle this case with ease:

```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1);

let loopback = IpAddr::V6(String::from("::1"));
```

You can put any kind of data inside an enum variant: strings, numeric types, or structs, or even enums in enums, like this:

```rust
enum Message {
    Quit, // No data associated
    Move { x: i32, y: i32 }, // An anonymousv struct inside it
    Write(String), // Single string
    ChangeColor(i32, i32, i32), // includes three i32 values
}
```

We're also abel to define methods on enums. For example here's a method that we could define for our `Message` enum:

```rust
    impl Message {
        fn call(&self) {
            // method body
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();
```

## The `Option` Enum and Its Advantages Over Null Values

The `Option` type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.

Rust doesn’t have the null feature that many other languages have. _Null_ is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

Rust has an enum that can encode the concept of a value being present or absent, this enum is `Option<T>`.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

This code won’t compile because it’s trying to add an i8 to an `Option<i8>`:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

Rust doesn’t understand how to add an `i8` and an `Option<i8>`, because they’re different types.

You have to convert an `Option<T>` to a T before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
