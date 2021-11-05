fn main() {
    println!("Hello, world!");
    xxx(111, 'o');

    let x = 5;
    let y = {
        let x = 4;
        x + 1
    };
    println!("The value of y is {}", y);
}

fn xxx(x: i32, y: char) {
    println!("This xxx function, parameter is {} {}", x, y);
}
