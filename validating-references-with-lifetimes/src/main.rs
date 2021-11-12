fn main() {
    // dangling reference
    // let r;
    // {
    //     let x = 5;
    //     // r is referring to has gone out of scope
    //     r = &x;
    // }

    // println!("r: {}", r);

    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    let s1 = String::from("abcd");
    let ret;
    {
        let s2 = "xyz";
        ret = longest(s1.as_str(), s2);
        // error:
        // We’ve told Rust that the lifetime of the reference returned by the longest
        // function is the same as the smaller of the lifetimes
        // of the references passed in
        // the lifetime of s1 is longer than s2
        // let s2 = String::from("xyz");
        // ret = longest(s1.as_str(), s2.as_str());
    }
    println!("The longest string is {}",  ret);
    // Lifetime annotations
    // the names of lifetime parameters must start with an apostrophe (')
    // and are usually all lowercase and very short
    // Most people use the name 'a
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // This annotation means an instance of ImportantExcerpt
    // can’t outlive the reference it holds in its part field.
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl <'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        // we’re not required to annotate the lifetime of the reference
        // to self because of the first elision rule
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i.part = {}", i.part);
    println!("i.level() = {}", i.level());
    i.announce_and_return_part("hello");

    // `'static`
    // means that this reference can live for the entire duration of the program
    // All string literals have the 'static lifetime
    let s: &'static str = "hello";
    println!("s is {}", s);

    longest_with_an_announcement("123", "11", "hello");

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // error:
    // it doesn’t know how the lifetimes of x and y relate to the
    // lifetime of the return value
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;
// lifetimes are a type of generic
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// error:
// result goes out of scope and gets cleaned up at the end of the longest function
// it is a dangling reference
// the best fix would be to return an owned data type
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// elision rules:
// 1. each parameter that is a reference gets its own lifetime parameter
// ```rs
// fn foo<'a>(x: &'a i32);
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
// ```
// 2. if there is exactly one input lifetime parameter,
//    that lifetime is assigned to all output lifetime parameters
// ```rs
// fn foo<'a>(x: &'a i32) -> &'a i32
// ```
// 3. if there are multiple input lifetime parameters,
//    but one of them is &self or &mut self because this is a method,
//    the lifetime of self is assigned to all output lifetime parameters.
