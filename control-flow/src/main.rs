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
    // error
    // if else must has the same value type
    // let number = if true { 4 } else { "six" };
    println!("The value of number is {}", number);

    let mut count = 0;
    'count_loop: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining is {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count is {}", count);

    count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("The result is {}", result);

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", a[index]);
        index += 1;
    }
}
