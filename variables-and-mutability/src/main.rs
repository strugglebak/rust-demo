fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is {}", THREE_HOURS_IN_SECONDS);

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    // 使用 let 可以改变变量的值以及类型(shadow)
    let spaces = "          ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);

    // 但是使用 mut 后就不能改变变量类型了
    // let mut spaces = "      ";
    // spaces = spaces.len();
}
