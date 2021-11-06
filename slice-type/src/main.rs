fn main() {
    let mut s = String::from("hello world");
    let word = first_world(&s);
    println!("word is '{}'", word);

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    // start at zero index
    // means [0..2]
    let slice = &s[..2];
    println!("{}", slice);

    // end at len index
    // means [3..len]
    let len = s.len();
    let slice = &s[3..];
    println!("{}, {}", slice, len);

    // get entire string
    let slice = &s[..];
    println!("{}", slice);

    // let s = "hello world";
    // s type is '&str'
    // which is a immutable reference

    s.clear();
}

// fn first_world(s: &String) -> &str {
// allow both &String and &str parameter
fn first_world(s: &str) -> &str {
    // convert string to an array of bytes
    let bytes = s.as_bytes();

    // bytes array iterator
    // enumerate wraps the result of bytes.iter()
    // and return a tuple
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' means space
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
