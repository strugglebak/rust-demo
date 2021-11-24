fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(1);
    stack.push(1);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3);
    //error:
    // let (x, y) = (1, 2, 3);

    let point = (x, y);
    print_coordinates(&point);

    // let statements, and for loops can only accept irrefutable patterns
    // The if let and while let expressions accept refutable and irrefutable patterns

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        // If x is 1, 2, 3, 4, or 5, the first arm will match
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point {x: a, y: b} = p;
    let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    assert_eq!(0, x);
    assert_eq!(7, y);
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        // ChangeColor(i32, i32, i32),
        ChangeColor(Color),
    }
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    // let msg = Message::ChangeColor(0, 160, 255);
    // match msg {
    //     Message::Quit => {
    //         println!("The Quit variant has no data to destructure.")
    //     }
    //     Message::Move { x, y } => {
    //         println!(
    //             "Move in the x direction {} and in the y direction {}",
    //             x, y
    //         );
    //     }
    //     Message::Write(text) => println!("Text message: {}", text),
    //     Message::ChangeColor(r, g, b) => println!(
    //         "Change the color to red {}, green {}, and blue {}",
    //         r, g, b
    //     ),
    // }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    foo(3, 4);

    let mut s = Some(5);
    let new_s = Some(10);
    match (s, new_s) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            s = new_s;
        }
    }
    println!("setting is {:?}", s);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    let s = Some(String::from("Hello!"));

    // error:
    // the s value will still be moved into _s
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);


    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    // can only be used once per tuple pattern
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
