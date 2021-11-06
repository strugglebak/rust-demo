fn main() {
    let mut s = String::from("hello world");
    let word = first_world(&s);
    s.clear();

    println!("word is '{}'", word);
}


fn first_world(s: &String) -> usize {
    // convert string to an array of bytes
    let bytes = s.as_bytes();

    // bytes array iterator
    // enumerate wraps the result of bytes.iter()
    // and return a tuple
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' means space
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
