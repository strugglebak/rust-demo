#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// using generic types in enum definitions
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// using generic types in function definitions
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn main() {
    let integer = Point { x: 5, y: 5.0 };
    let float = Point { x: 1.0, y: 1 };
    println!("{:?}", integer);
    println!("{:?}", float);
}
