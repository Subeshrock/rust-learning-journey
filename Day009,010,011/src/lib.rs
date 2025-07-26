// Library crate
// This is the library crate that can be used by other crates.
// It can contain functions, structs, enums, and other items that can be shared.

// mod front_of_house;
// this way we are deifning a module named front_of_house
// the content or the deifinition of the module is in the file named front_of_house.rs


// module declarations mod is used to declare modules.
// Modules can be used to organize code and control visibility.
mod front_of_house {

    // this is a submodule within the front_of_house module
    // pub keyword makes the module public
    // so that it can be accessed from outside this module
    // by default, modules are private
    pub mod hosting {
        // pub keyword makes the function public
        // so that it can be accessed from outside this module
        // by default, functions are private
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }
        
        fn seat_at_table() {
            println!("Seating at table");
        }
    }

    // another submodule within the front_of_house module
    mod serving {
        fn take_order() {
            println!("Taking order");
        }
        
        fn serve_order() {
            println!("Serving order");
        }

        fn take_payment() {
            println!("Taking payment");
        }
    }
}

fn serve_order() {
    println!("Serving order from the library crate");
}

mod back_of_house {

    fn fix_incorrect_order() {
        cook_order();
        // Calling serve_order from the parent module
        // super is used to refer to the parent module
        // This allows us to access items from the parent module
        super::serve_order();
        println!("Fixing incorrect order");
    }

    // This is a private module
    // It can only be accessed from within this crate
    fn cook_order() {
        println!("Cooking order");
    }

    pub fn prepare_order() {
        cook_order();
        println!("Order prepared");
    }

    pub struct Breakfast {
        // This is a public field
        // It can be accessed from outside this module
        // even though the struct itself is public the field are private by default
        // so we need to use pub keyword to make it public
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // This is a public function
        // It can be accessed from outside this module
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // This is a public enum
    // It can be accessed from outside this module
    // default visibility for enums is private
    // fields of the enum are public if enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}