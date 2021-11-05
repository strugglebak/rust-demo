fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // error
    // let guess = "42".parse().expect("Not a number!");

    // error integer overflow
    // let guess: u8 = 257;
    println!("guess value is {}", guess);
}
