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

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::mem::drop;

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

    // Because we implemented the Deref trait on MyBox<T>
    // Rust can turn &MyBox<String> into &String by calling deref
    // Becaust the standard library provides an implementation of Deref on String
    // Rust calls deref again to turn the &String into &str
    let m = MyBox::new(String::from("Rust"));
    // more like
    // hello(&(*m)[..]);
    // (*m): MyBox<String> -> String
    // & [..]: &string -> &str
    hello(&m);

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.");

    // error:
    // explicit destructor calls not allowed
    // because Rust would still automatically call drop on the value at the end of main.
    // This would be a double free error
    // because Rust would be trying to clean up the same value twice.

    // _c.drop();

    // force to be dropped early as an argument
    drop(_c);

    println!("CustomSmartPointer dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// you can use the DerefMut trait
// to override the * operator on mutable references.
// Rust does deref coercion when it finds types and
// trait implementations in three cases:
// 1. From `&T` to `&U` when `T: Deref<Target=U>`
// 2. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
// 3. From `&mut T` to `&U` when `T: Deref<Target=U>`
