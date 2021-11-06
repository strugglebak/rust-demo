fn main() {
    let s1 = String::from("hello");
    let l = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, l);

    // will be error
    // not allowed to modify sth we have a reference to
    // let s2 = String::from("hello");
    // change(&s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &String) {
    s.push_str(", world")
}
