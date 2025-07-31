use std::env;
use std::process;

use simgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = args.get(1).unwrap_or(&"default_query".to_string());

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = simgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}