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

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// this will be OK
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 5, y: 5.0 };
    let float = Point { x: 1.0, y: 1 };
    let string = Point {x: 'x', y: "hello world "};
    println!("{:?}", integer);
    println!("{:?}", float);
    println!("{} : {}", float.x(), float.y());

    let p = integer.mixup(string);
    println!("{:?}", p);
}
