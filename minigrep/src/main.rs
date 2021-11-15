use std::env;

fn main() {
    // iterators produce a series of values,
    // and we can call the collect method on an iterator to turn it into a collection,
    // Note that std::env::args will panic if any argument contains invalid Unicode
    // you can use std::env::args_os instead
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
