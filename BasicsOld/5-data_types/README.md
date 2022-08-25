# Data types

Two data type subset: scalar and compound.
Rust is a statically typed language, that means that it must know the types of all the variables at compile time.

Scalar types:
A scalar type represents a single value. Rust has 4 primary scalar types: integers, floating-point numbers, booleans and characters.

## Integer types

An integer is a number without a fractional component. We can use unsigned and signed integers and use different bytes sizes.

| Length  | Signed | Unsigned |
| :-----: | :----: | :------: |
|  8-bit  |   i8   |    u8    |
| 16-bit  |  i16   |   u16    |
| 32-bit  |  i32   |   u32    |
| 64-bit  |  i64   |   u64    |
| 128-bit |  i128  |   u128   |
|  arch   | isize  |  usize   |

The `isize` and `usize` depends the kind of computer your program is running on: 64bits if you are on x64 architecture
or 32bits if you are on x32 architecture.

---

Integer literals: you can write integer literals in any form as next

| Number literals |   Example   |
| :-------------: | :---------: |
|     Decimal     |   98_222    |
|       Hex       |    0xff     |
|      Octal      |    0o77     |
|     Binary      | 0b1111_0000 |
| Byte (u8 only)  |    b'A'     |

## Floating-point types

Rust has two primitive types for floating-point numbers, these types are `f32` and `f64`, which are 32 and 64 bits in size respectively. The `f32` is a single-precision float and `f64` is a double-precision float.

Rust has all the common mathematical operations like other languages.

## Boolean types

A boolean type has two posible values: `true` and `false`. Booleans are one byte in size. A boolean is specified with `bool` keyword.

## Character type

Rust `char` is the most primitive alphabetic type. Char literals are specified with single quotes. `let c = 'a';`, A char in rust not match completely the definition of a character.

## Compound types

Compound types can group multiple values into one type. Rust has two primitive compound types:tuples and arrays.

### The tuple type

A tuple allow group multiple values of different types in a compound type. Tuples are fixed and cannot be resized. We write tuples with `let tup: (i32, f64, u8) = (500, 6.4, 1);`. As you can see each position in the tuple has their own type.

To get individual values from a tuple we can use pattern matching to destructure a tuple value, like this:

```rust
 let tup = (5000, 6.4, 1);
 let (x, y , z) = tup; // <= tuple destructuring or destructuration
 println!("The value of y is {}", y);
```

In addition to destructuring we can access to the elements of a tuple using the index and period`.`

```rust
    let x = (i32, i64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_position_four = x.1;
    let one = x.2;
```

### The type array

Unlike a tuple, in an array every element must have the same type. Arrays has fixed length, to create an array we write as a comma-separated list insides square brackets `let a = [1,2,3,4,5];`.

Arrays are useful when you want your data allocated on the **stack** rather than the **heap**.

You can also create an array including the element types like `let a: [u32;5] = [1,2,3,4,5];`

Also if you want to initialize an array with a value in all their elements you can do that with `let a:[3;5];` is the same as `let a: [u32;5] = [3,3,3,3,3];` but in a more concise way.

To access the elements on an array you can use indexing, like this:

```rust
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
```

Try to access an invalid index will cause a **_runtime_** error.
