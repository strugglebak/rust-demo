fn main() {
    let number = 6;
    // number must be a boolean
    // it will be error !
    // if number {
    if number < 5 {
        println!("number < 5");
    } else if number == 5 {
        println!("number == 5");
    } else {
        println!("number >= 5");
    }

    let number = if true { 4 } else { 5 };
    println!("The value of number is {}", number);
}
