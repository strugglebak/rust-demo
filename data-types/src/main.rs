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

    let t = true;
    let f: bool = false;
    println!("t is {}, f is {}", t, f);

    let tup: (i32, f64, u8) = (500, 0.3, 1);
    let (_x, _y, _z) = tup;
    println!("The value of y is: {}", _y);
    println!("The first value of tup is: {}", tup.0);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] is {}", a[0]);
    let a = [3; 5];
    println!("a[0] is {}", a[0]);
}
