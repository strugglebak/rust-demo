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
}
