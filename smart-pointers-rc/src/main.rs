
use crate::List::{Cons, Nil};
use std::rc::Rc;

// Using Rc<T> allows a single value to have multiple owners
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
// The RefCell<T> type is useful when you’re sure your code follows the borrowing
// rules but the compiler is unable to understand and guarantee that
//
// the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
//
// 1.
// Rc<T> enables multiple owners of the same data;
// Box<T> and RefCell<T> have single owners.
//
// 2.
// Rc<T> allows only immutable borrows checked at compile time;
// Box<T> allows immutable or mutable borrows checked at compile time;
// RefCell<T> allows immutable or mutable borrows checked at runtime.
//
// 3.
// Because RefCell<T> allows mutable borrows checked at runtime,
// you can mutate the value inside the RefCell<T>
// even when the RefCell<T> is immutable.

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // The call to Rc::clone only increments the reference count
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of the scope = {}", Rc::strong_count(&a));

    // let x = 5;
    // error:
    // cannot borrow as mutable
    // let y = &mut x;
}
