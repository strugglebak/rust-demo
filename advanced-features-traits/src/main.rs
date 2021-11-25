use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// The default generic type in this code is within the Add trait
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // without fmt::Display, to_string() will be error
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}


// error:
// Display is required but not implemented
impl OutlinePrint for Point {}
// fix it by
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
// we can call outline_print on a Point instance
// to display it within an outline of asterisks

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    // This is your captain speaking.
    // Up!
    // *waving arms furiously*
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // A baby dog is called a Spot
    println!("A baby dog is called a {}", Dog::baby_name());
    // error:
    // Because Animal::baby_name is an associated function rather than a method,
    // and thus doesn’t have a self parameter,
    // Rust can’t figure out which implementation of Animal::baby_name we want.

    // associated functions that are part of traits don’t have a self parameter
    // When two types in the same scope implement that trait
    // Rust can’t figure out which type you mean unless you use fully qualified syntax

    // println!("A baby dog is called a {}", Animal::baby_name());

    // using fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // In general, fully qualified syntax is defined as follows:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
}
