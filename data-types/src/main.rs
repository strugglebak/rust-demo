fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // error
    // let guess = "42".parse().expect("Not a number!");

    // error integer overflow
    // let guess: u8 = 257;
    println!("guess value is {}", guess);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("x is {}, y is {}", x, y);
}
