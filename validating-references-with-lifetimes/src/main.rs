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
