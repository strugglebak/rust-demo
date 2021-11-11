#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 5.0 };
    let float = Point { x: 1.0, y: 1 };
    println!("{:?}", integer);
    println!("{:?}", float);
}
