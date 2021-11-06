fn main() {
    {
        let s = "hello world";
        // s is valid here(in this scope)
        println!("The value of s is {}", s);
    }
    // error: s is not valid
    // println!("The value of s is {}", s);

    // String can be mutated but string literals can not
    let mut s = String::from("hello world");
    s.push_str(", ni hao");
    println!("{}", s);

    {
        // Resource Acquisition is Initialization
        // s here is valid
        let _s = String::from("hello world");
    }   // the scope is over, s is no longer valid
        // and call the drop function

    let s1 = String::from("hello");
    // just copy the reference of s1
    // let _s2 = s1;

    // error: s1 no longer be valid
    // println!("{}, world", s1);

    let s3 = s1.clone();
    println!("s1 = {}, s3 = {}", s1, s3);

    takes_ownership(s1);

    // Stack-Only data: copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    makes_copy(x);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(num: i32) {
    println!("{}", num);
}
