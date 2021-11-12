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
    let s2 = "xyz";

    let ret = longest(s1.as_str(), s2);
    println!("The longest string is {}",  ret);
}

fn longest(x: &str, y: &str) -> &str {
    // error:
    // it doesn’t know how the lifetimes of x and y relate to the
    // lifetime of the return value
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
