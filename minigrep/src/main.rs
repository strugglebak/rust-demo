use std::{env, fs, process};
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str>  {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

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

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// Box<dyn Error> means the function will return a type that implements the Error trait
// dyn means dynamic
// This gives us flexibility to return error values
// that may be of different types in different error cases.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    // using () like this is the idiomatic way to indicate
    // that we’re calling run for its side effects only
    // it doesn’t return a value we need
    Ok(())
}
