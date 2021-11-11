fn main() {
    // let mut str = String::new();

    let data = "hello world";
    let _s = data.to_string();
    // let _s = "hello world".to_string();

    let _s = String::from("hello world");

    // stings are UTF-8 encoded
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    let mut hello = String::from("Hola");
    hello.push_str("world");

    let mut s1 = String::from("hello");
    let s2 = "world";
    // push_str doesn't take ownership of the s2
    s1.push_str(s2);
    println!("s2 is {}", s2);

    s1.push('X');
    println!("s1 is {}", s1);

    // + operator looks like this
    // fn add(self, s: &str) -> String {
    // but the type of `&s2` is `&String`, not the `&str`
    // so + operator force the `&String` argument into a `&str`
    // turns `&s2` into `&s2[...]`
    // + operator take ownership of `self`, because `self` doesn't have an `&`
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    // error:
    // s1 has been moved here and can no longer be used
    // println!("s1 is {}", s1);
    let s4 = String::from("4");
    let s5 = String::from("5");
    let s6 = String::from("6");
    let s7 = s4 + "-" + &s5 + "-" + &s6;
    println!("s7 is {}", s7);
    // use format! macro
    // and this call doesn’t take ownership of any of its parameters
    let s8 = format!("{}-{}-{}", s5, s6, s7);
    println!("s8 is {}", s8);
}
