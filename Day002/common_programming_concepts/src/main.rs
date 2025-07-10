fn main() {
    data_types()
    functions_kind()
    branches()
    kind_of_loops()
}

fn data_types() {
    let guess: i32 = 42

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    let mut j: (i32, i32) = (1, 2);
    j.0 = 0;
    j.1 += 5;

    // Array Type

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // An array of 5 i32 elements
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5]; // An array of 5 elements, all initialized to 3

    let first = a[0];
    let second = a[1];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
}


fn functions_kind() {
    fn another_function() {
        println!("Another function.");
    }

    fn function_with_parameter(x: i32) {
        println!("The value of x is: {x}");
    }

    // Function with multiple parameters
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    // Function with only statement
    fn func_with_statement() {
        let y = 6;
    }

    // Function with return value
    fn five() -> i32 {
        5
    }

    // Function with parameter, return value and expression
    // The return value is the last expression in the function body
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
}

// This is a Comment

//This is also a comment
//This a way to write multiple line comments

/*
This is a block comment
This is a way to write multiple line comments
 */

/// This is a documentation comment

fn branches() {
    let number = 6;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // If statement with multiple conditions
    if number < 5 {
        println!("Condition was true");
    } else if number == 5 {
        println!("Condition was equal to 5");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // Using a loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
}

fn kind_of_loops() {
    // Infinite Loop
    loop {
        println!("again!");
    }

    let mut counter = 0;

    // General loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loop

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // For Loop

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for rev_number in (1..4).rev() {
        println!("{rev_number}!");
    }
    println!("LIFTOFF!!!");
}