// this is a submodule within the front_of_house module
// pub keyword makes the module public
// so that it can be accessed from outside this module
// by default, modules are private

// we can write this module like -
// pub mod hosting;
// by sending the content of this file to the hosting file
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