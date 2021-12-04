
use std::fmt;

type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    // fn flush(&mut self) -> Result<(), Error>;

    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    // The main use case for type synonyms is to reduce repetition
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    // error:
    // Rust needs to know how much memory to allocate for any value of a particular type,
    // and all values of a type must use the same amount of memory
    // we make the types of s1 and s2 a &str rather than a str
    // The golden rule of dynamically sized types is that
    // we must always put values of dynamically sized types behind a pointer of some kind.
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}

// Dynamically Sized Types
// To work with DSTs, Rust has a particular trait called the Sized trait
// to determine whether or not a type’s size is known at compile time
// ?Sized means “T may or may not be Sized”
fn generic<T: ?Sized>(t: &T) {
    // &T
    // because the type might not be Sized,
    // we need to use it behind some kind of pointer
    // --snip--
}
fn generic2<T: Sized>(t: T) {
    // --snip--
}



fn f1(f: Thunk) {
}
// fn f2() -> Thunk {
    // ...
// }

// the function bar returns never
fn bar() -> ! {
    // --snip--
}
