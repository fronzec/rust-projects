enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}


enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {

// Simple enum
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;


// Enum on functions
route(IpAddrKind::V4);
route(IpAddrKind::V6);

// Enum inside an struct
let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

// Enums with same types
let home = IpAddrEnum::V4(String::from("127.0.0.1"));

// Enums with different types
let home = IpAddrEnum2::V4(127, 0, 0, 1);
let loopback = IpAddrEnum2::V6(String::from("::1"));

// Methods on enums
let m = Message::Write(String::from("hello"));
m.call();

// The Option enum
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
}

fn route(ip_kind: IpAddrKind) { }
