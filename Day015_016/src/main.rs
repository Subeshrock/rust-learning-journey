use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");

    a();

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    enum_example();
}

fn a() {
    b();
}

fn b() {
    c(0);
}

fn c(num: i32) {
    if num == 0 {
        panic!("crash and burn");
    }
}

fn enum_example() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("File not found, creating a new one...");
                match File::create("hello.txt") {
                    Ok(new_file) => println!("File created successfully: {:?}", new_file),
                    Err(e) => println!("Failed to create file: {}", e),
                }
            }
            other_error => println!("Problem opening the file: {:?}", other_error),
        },
    }

    let f = File::open("hello.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Failed to create file: {}", e);
            })
        } else {
            panic!("Problem opening the file: {:?}", e);
        }
    });

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open file");

    // let f = File::open("hello.txt")?; // This line would be used in a function that returns a Result type
}