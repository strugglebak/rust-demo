use std::{env, process};
use minigrep::Config;

fn main() {
    // iterators produce a series of values,
    // and we can call the collect method on an iterator to turn it into a collection,
    // Note that std::env::args will panic if any argument contains invalid Unicode
    // you can use std::env::args_os instead
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // Using unwrap_or_else allows us to define some custom, non-panic! error handling
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
