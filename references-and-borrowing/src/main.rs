fn main() {
    let s1 = String::from("hello");
    let l = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, l);

    // will be error
    // not allowed to modify sth we have a reference to
    // let s2 = String::from("hello");
    // change(&s2);

    let mut s2 = String::from("hello");
    change(&mut s2);

    println!("s2 is {}", s2);

    // will be error
    // only one mutable reference can assign to a variable at a time
    // prevent data race
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // will be error
    // let mut s = String::from("hello");
    // let r1 = &s;      // ok
    // let r2 = &s;      // ok
    // let r3 = &mut s;  // error!!!
    // println!("{}, {}, {}", r1, r2, r3);

    let mut s = String::from("hello");
    let r1 = &s;      // ok
    let r2 = &s;      // ok
    println!("{}, {}", r1, r2);
    let r3 = &mut s;  // ok
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
//     s.push_str(", world")
// }

fn change(s: &mut String) {
    s.push_str(", world")
}
