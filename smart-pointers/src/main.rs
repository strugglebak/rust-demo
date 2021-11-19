use crate::List::{Cons, Nil};
// error:
// Rust can’t figure out how much space it needs to store a List value
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(4);
    println!("b = {}", b);

    // error:
    // let _list = Cons(1, Cons(2, Cons(3, Nil)));
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
