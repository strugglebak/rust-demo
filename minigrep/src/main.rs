use std::{env, fs};

struct Config {
    query: String,
    filename: String,
}

fn main() {
    // iterators produce a series of values,
    // and we can call the collect method on an iterator to turn it into a collection,
    // Note that std::env::args will panic if any argument contains invalid Unicode
    // you can use std::env::args_os instead
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let (query, filename): (&str, &str) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
