#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// use generic types in enum definitions
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    let integer = Point { x: 5, y: 5.0 };
    let float = Point { x: 1.0, y: 1 };
    println!("{:?}", integer);
    println!("{:?}", float);
}
