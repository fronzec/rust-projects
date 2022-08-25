# Structures

A _struct_ or _structure_ is like an object's data attributes.
Is a custom data type that lets you name and package related values.

Structs are similar to tuples, like tuples, the pieces of a struct can be a different types, unlike with tuples structs you don't have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and give a name. Then inside curly brackets we define the names and types of the pieces of data called _fields_, like this:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
} // without semicolon
```

To use a struct , we create an _instance_ of that struct by specifying concrete values for each field. We create an instance by stating the name of the struct and then add curly brackets containing `key:value` pairs, when the _keys_ are the name of the fields and the values are the data we want to store on those fields. The order is irrelevant. In other words, the struct is like a general template of the type and the instances fill that template with data to create values of that structure type. For example:

```rust
// As you can see the order of the fields is irrelevant
// 'user1' is an instance of User struct type
let user1 = User {
    email: String::from("user@test.com"),
    username: String::from("user"),
    active: true,
    sign_in_count: 1,

};
```

To access a value of a struct, we can use _dot notation_. Like this:

```rust
// Get the value of one field
let username = user1.username;

// If the instance is mutable we can change a field like this
let mut user1 = User {
    //user initialization values
 };

user1.username = String::from("username update");

```

**When an instance is is marked as mutable, the entire instance is mutable**, Rust doesn't allow us to mark only certain fields as mutable.

Take a look on this example:

```rust
fn build_user(username:String, email:String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    } // Return a new User instance using the given information
}
```

It makes sense to name the function parameters with the same name as the struct fields, but having to repeat the `email` and `username` field names and individual variables is a bit tedious. Luckily there's a convenient shorthand. Using the _field init shorthand_ when variables and fields have the same name.

We can do this:

```rust
fn build_user(username:String, email:String) -> User {
    User {
        username,
        email,
        active:true,
        sign_in_count: 1,
    }
}
```

Using `email` is the shorthand for `email: email` because the field and email parameter have the same name.

## Creating instances from other instances with struct update syntax

To create a new instance of a struct that uses an old instance's values but change some we can use _struct update syntax_.

The next examples are equal:

```rust
// Without struct update syntax
let user2 = {
    email : String::from("another@example.com"),
    username: String::from("other12345"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

// Using struct update syntax
let user2 = User {
    email: String::from("another@example.com"),
    username :String::from("other12345"),
    ..user1 // <- Struct Update syntax
};

```

## Structs without named fields to create different types

Also you can define structs that looks similar to tuples, called _tuple structs_ this structs doesn't have names associated with their fields; rather, they just have the type of the fields.

To define a tuple struct, start with the `struct` keyword and the struct name followed by the types int the tuple. e.g.:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0,0,0);
let origin = Point(0,0,0);
```

`black` and `origin` are different types, Each struct you define is its own type. a function that takes a parameter of type `Color` cannot take a `Point` argument, even though both types are made up of three `i32` values. Like tuples you can destructure them into their individual pieces. To access use the dot notation `.`.

## Unit-Like structs without any fields

You can also define structs that don't have any fields. These are called _unit-like structs_ because they behave similar to `()`, the unit type. It can be useful in situations in which you need to implement a trait in some type but don't have any data that you want to store.

## Ownership of struct data

In the `User` struct we use `String` type, this makes each struct instance own all of its data and for that data to be valid for as long as the entire struct is valid.

It's possible to store references (`&str` types) dto data owned by something else, but to do so requires the use of _lifetimes_. Lifetimes ensure that data referenced by a struct is valid for as long as the struct is.
