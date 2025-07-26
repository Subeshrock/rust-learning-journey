// Root Binary Crate
// This is the main entry point for the binary crate.
// It can use the library crate defined in `lib.rs`.
// The library crate can be used by other crates as well.

use crate::front_of_house::hosting;

fn main() {
    println!("Hello, world!");

    // Using functions from the front_of_house module
    // This is absolute path to the module
    crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::seat_at_table();

    // This is relative path to the module
    front_of_house::serving::take_order();

    // This is how we call a function from the back_of_house module usinf use at the top by brigning it into scope
    serving::serve_order();
    serving::take_payment();
}

use std::fmt;
use std::io;

//both have result type
// fmt::Result and io::Result are different types
// we have imported the fmt and io which parent modules so that we do not have conflict between both the Result types
// we can use them in the same scope without conflict

// This is one more syntax to bring same types in scope withot conflict
// use std:fmt::Result as FmtResult;
// use std::io::Result as IoResult;
// This is another way of impoting multiple items from the same module
// use std::{fmt, io}
// use std::io::{self, Write, Read}; self is used to refer to the current module
// another sytax to import everything which is public from the module
// use std::io::*;

fn function1() -> fmt::Result {
    println!("Function1 from fmt module");
    Ok(())
}

fn function2() -> io::Result<()> {
    println!("Function2 from io module");
    Ok(())
}
