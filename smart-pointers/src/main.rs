use crate::List::{Cons, Nil};
// error:
// Rust can’t figure out how much space it needs to store a List value
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// The Box<T> type is ultimately defined as a tuple struct with one element
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(4);
    println!("b = {}", b);

    // error:
    // let _list = Cons(1, Cons(2, Cons(3, Nil)));
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    // error:
    // To enable dereferencing with the * operator, we implement the Deref trait.
    // the same as *(a.deref())
    assert_eq!(5, *a);
}
