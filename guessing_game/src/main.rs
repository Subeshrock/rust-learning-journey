use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::rng().random_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number.".blue());
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
        }
    }
}
