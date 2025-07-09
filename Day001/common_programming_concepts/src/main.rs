fn main() {
    // println!("Hello, world!");
    variables()
}

fn variables() {
    let immut = 5; // Immutable variable
    // x = 6; // This line would cause a compile-time error because x is immutable
    let mut x = 5; // Mutable variable
    // x = 6; // This line is now valid because x is mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Constant variable
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing example
    let x = 5; // Immutable variable
    let x = x + 1; // Shadowing the previous variable
    
    {
        let x = 10; // New scope with a new x
        println!("The value of x after shadowing in the inner scope is: {x}");
    }
    println!("The value of x after shadowing is: {x} in outer scope");

    let spaces = "   ";
    let spaces = spaces.len();
}