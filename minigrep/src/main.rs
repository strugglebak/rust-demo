use std::{env, fs, process};

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

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
