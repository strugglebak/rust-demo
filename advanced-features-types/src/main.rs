
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
