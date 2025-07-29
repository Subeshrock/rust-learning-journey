fn main() {
    let r;

    {
        let x = 5;
        r = &x; // r borrows x
    }

    println!("r: {}", r); // This will cause a compile-time error because x is no longer valid
    // println!("x: {}", x); // This will also cause a compile-time error because x is no longer valid

    let string1: = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);

    // Demonstrating static lifetime
    // Static lifetime is a special lifetime that lasts for the entire duration of the program.
    // It is used for string literals and other data that is hardcoded into the binary.
    // All the string literals have a static lifetime.
    let s: &'static str = "I have a static lifetime";
    println!("Static string: {}", s);
}

// this function have same lifetime for both parameters
// and return type
fn same_lifetime() {
    let string1: = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);
}

// this function have different lifetime for both parameters
// and return type but result will be valid because shortest lifetime of second parameter is still in the scope
// so it will not cause any compile-time error
// but it is not recommended to use this way because it can lead to confusion
fn different_lifetime() {
    let string1: = String::from("abcd");

    {
        let string2 = String::from("xyz");
        
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }
}

fn diffrent_life_scope() {
    let string1: = String::from("abcd");

    {

        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is: {}", result);

    // This will cause a compile-time error because string2 is no longer valid
    // after the block ends, but we are trying to use it outside the block.
}
// &i32          // a reference
// &'a i32       // a reference with an explicit lifetime 'a
// &'a mut i32   // a mutable reference with an explicit lifetime 'a
// &'a str       // a string slice with an explicit lifetime 'a
// &'a mut str   // a mutable string slice with an explicit lifetime 'a
// &'a [i32]     // a slice of i32 with an explicit lifetime 'a
// &'a mut [i32] // a mutable slice of i32 with an explicit lifetime 'a

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// this function will work for diffrent_life_scope function because we do not dippend on the second parameter's lifetime
// fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     x
// }

// fn longest<'a>(s1: & str, s2: & str) -> &'a str {
    // let result = String::from("Hello, world!");
    // result.as_str() // This will cause a compile-time error because result is dropped at the end of the function
// }

// This will work because we are returning a String
// fn longest<'a>(s1: & str, s2: & str) -> String {
    // let result = String::from("Hello, world!");
    // result
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&'a self, announcemnet: &str) -> &'a str {
        println!("Attention please: {}", announcemnet);
        self.part
    }
}

fn struct_lifetime() {
    let s: String = String::from("Hello, world!");
    let first_sentence = s.split('.').next().expect("Could not find a '.' in the string");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt part: {}", i.part);
}

// Rules of lifetimes:
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.

// Input lifetime means the lifetime of the reference passed to the function, and output lifetime means the lifetime of the reference returned by the function.