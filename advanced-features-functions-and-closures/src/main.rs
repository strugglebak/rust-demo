fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings : Vec<String> =
        list_of_numbers.iter()
        // .map(|i| i.to_string())
        .map(ToString::to_string)
        .collect();

    enum Status {
        // These types use () as initializer syntax,
        // which looks like a function call
        Value(u32),
        Stop,
    }
    // creating Status::Value instances
    // using the initializer function of Status::Value
    let list_of_statuses: Vec<Status> =
        (0u32..20).map(Status::Value).collect();
}
