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
}
