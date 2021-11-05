fn main() {
    println!("Hello, world!");
    xxx(111, 'o');

    let _x = 5;
    let y = {
        let x = 4;
        x + 1
    };
    println!("The value of y is {}", y);

    println!("The five function return {}", five());
}

fn xxx(x: i32, y: char) {
    println!("This xxx function, parameter is {} {}", x, y);
}

fn five() -> i32 { 5 }
