fn main() {
    println!("Hello, world!");
}

fn structs() {
    // Define a struct
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_count: u64,
    }

    // Create an instance of the struct
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("test@testmail.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("user1Change")

    // Accessing fields of the struct
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign in count: {}", user1.sign_in_count);
    // Create another instance using struct update syntax
    let user2 = User {
        username: String::from("user2"),
        email: String::from(""),
        ..user1 // Use the values from user1 for the rest of the fields
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black color RGB: ({}, {}, {})", black.0, black.1, black.2);

    // Unit-like Structs Without Any Fields
    struct UnitStruct;
    let _unit = UnitStruct; // Instance of a unit-like struct

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1
        }
    }

    let user3 = build_user(String::from("test@test.com", String::from("user3")))
}