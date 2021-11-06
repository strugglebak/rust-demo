fn main() {
    {
        let s = "hello world";
        // s is valid here(in this scope)
        println!("The value of s is {}", s);
    }
    // error: s is not valid
    // println!("The value of s is {}", s);
}
