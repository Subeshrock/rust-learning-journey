// use std::option::Option; // Removed unnecessary import

// this is a enum example
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Removed custom Option<T> enum to use Rust's standard library Option

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function(&self) {
        // Example method for the Message enum
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
            Message::Write(text) => println!("Write message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6;

    let localhost = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };
    
    let some_number = Some(5);
    let some_char = Some('e');
    
    // let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = None;

    // let sum = x + y; // This line will cause a compile error because you cannot add i8 and Option<i8> directly
    let sum  = x + y.unwrap_or(0); // Correct way to handle Option

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(10);
    match some_value {
        Some(10) => println!("Matched the value 10!"),
        _ => println!("Did not match the value 10."),
    }

    if let Some(10) = some_value {
        println!("Matched the value 10 using if let!");
    } else {
        println!("Did not match the value 10 using if let.");
    }
}

fn route(ip_kind: IpAddrKind) {
    // Function to handle routing based on IP address kind
    match ip_kind {
        IpAddrKind::V4(_, _, _, _) => println!("Routing IPv4"),
        IpAddrKind::V6(_) => println!("Routing IPv6"),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is a penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This is a quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // Function to add one to an Option<i32>
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
