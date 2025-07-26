use std::collections::HashMap;

fn main() {
    let a: = [1, 2, 3];

    // This is an empty vector
    let v:Vec<i32> = Vec::new();

    // This is the way to insert elements into a vector
    v.push(1);
    v.push(2);
    v.push(3);

    {
        // This is a scope. when this scope ends, the vector will be dropped
        let v2 = vec![1, 2, 3]; // This is a macro to create a vector
    }

    let v3 = vec![1, 2, 3, 4, 5, 6];

    let third = &v3[2]; // This is how to access an element in a vector through indexing
    // let third = &v3[100]; // This will panic at runtime because the index is out of bounds
    println!("The third element is {}", third);

    // This is how to access an element in a vector safely
    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // This is how to iterate over a vector
    // &v3 is a reference to the vector, so we don't take ownership of it
    for i in &v3 {
        println!("The element is {}", i);
    }

    // This is how to iterate over a vector mutably
    // &mut v3 is a mutable reference to the vector, so we can modify its elements
    let mut v3 = v3; // We need to make v3 mutable to modify it
    for i in mut v3 {
        println!("The element is {}", i + 1);
    }
}

fn enum_in_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("The first element is an integer: {}", i),
        SpreadsheetCell::Float(f) => println!("The first element is a float: {}", f),
        SpreadsheetCell::Text(s) => println!("The first element is a text: {}", s),
    }
}

fn strings() {
    // Strings are stored ass a collection of UTF-8 encoded bytes
    let s1 = String::new(); // 
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s5 = String::from("Hello, ");
    let s6 = String::from("World!");
    let s7 = s5 + &s6;

    let s8 = format!("{}{}", s5, s6); // This will not cause panic becasue format doesn't take ownership.

    // println!("{}", s5) this will cause panic because s5 ownership have been moved to s7

    let hello: String = String::from("Hello");
}

fn hash_maps() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores HashMap::new()

    scores.insert(blue, 10);
    scores.insert(blue, 20);

    scores.insert(yellow, 50);
    scores.entry(yellow).or_insert(30);

    let team_name = String;:from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}