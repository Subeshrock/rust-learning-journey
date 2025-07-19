fn main() {
    // Ownership Rules
    // 1. Each value in Rust has a variable that is its "owner".
    // 2. A value can only have one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s in not valid here, it is not yet declared
        let s = "hello"; // s owns the string, s is valid from this point forward
        // do stuff with s
    } // this scope is over, and s is no longer valid

    let x = 5;
    let y = x; // copy

    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)

    // let s2 = s1.clone(); // This would create a deep copy of s1.

    println!("{}", s1); // This will cause an error because s1 is no longer valid

    let s =String::from("hello");
    takes_ownership(s); // s's value moves into the function and is no longer valid
    // println!("{}", s); // This will cause an error because s is no longer valid

    let x = 5;
    makes_copy(x);
    println!("{}", x); // This is valid because i32 is a Copy type, so x is still valid

    let s3 = gives_ownership(); // s3 now owns the string returned by the function
    println!("{}", s3); // This is valid because s3 is now the owner of the string

    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4); // s4 is moved into the function and s5 gets the ownership back
    println!("{}", s5); // This is valid


    let s6 = String::from("hello");
    // here s6 is borrowed, so it is still valid after this call, we are passing a reference to s6
    let len = calculate_length(&s6); // s6 is borrowed, so it is still valid after this call
    println!("The length of '{}' is {}.", s6, len); // This is valid

    let mut s7 = String::from("hello");
    // here we are passing a mutable reference to s7, so we can change it in the function
    change(&mut s7); // s7 is borrowed mutably

    let mut s8 = String::from("hello");
    let r1 = &mut s8; // r1 is a mutable reference to s8
    // let r2 = &mut s8; // This would cause an error because we cannot have two mutable references to the same value at the same time in same scope
    // println!("{}", r1); // This would cause an error because r1 is a mutable reference and we cannot use it


    let a = [1, 2, 3, 4, 5]; // a is an array of integers
    let  slice = &a[1..3]; // slice is a reference to a part of the array, it borrows the data
    println!("Slice: {:?}", slice); // This is valid, slice is a reference to a part of the array
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called

fn makes_copy(some_integer: i32) -> i32 {
    println!("{}", some_integer) // i32 is a Copy type, so it is not moved
} // some_integer goes out of scope but nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // returns the string and gives ownership to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string // returns the string and gives ownership back to the caller
}

// this paramters is a reference to a String
// it does not take ownership of the String, so the String is still valid after this call
// this is called borrowing
fn calculate_length(s: &String) -> usize {
    // s is immutable by default, so we cannot change it
    // if we want to change it, we need to use a mutable reference
    s.len() // returns the length of the string
} // s goes out of scope but nothing special happens because it is a reference

// this function takes a mutable reference to a String
// it can modify the String in place
// this is also called borrowing, but with a mutable reference
// we can only have one mutable reference to a value at a time
fn change(some_string: &mut String) {
    some_string.push_str(", world"); // modifies the string in place
} // some_string goes out of scope but nothing special happens because it is a mutable reference

// The rules of references
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.
// 3. References cannot outlive the data they point to.
// 4. References are immutable by default, but you can create mutable references using the `mut` keyword.
// 5. You cannot have a mutable reference while immutable references are active.
// 6. References are used to borrow data without taking ownership.
// 7. References are used to avoid unnecessary copying of data.
