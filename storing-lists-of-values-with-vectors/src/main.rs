fn main() {
    // v value changed, so we need to make it mutable
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // vec! is a macro
    {
        let _a = vec![1, 2, 3];
    } // _a is freed when is goes out of scope

    let mut b = vec![1, 2, 3, 4, 5];
    let third: &i32 = &b[2];
    println!("The third element is {}", third);

    match b.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // error: out of bound
    // let _does_not_exist = &b[100];
    // let _does_not_exist = b.get(100);

    let first = &b[0];
    // error:
    // push might require allocating new memory and copying the old elements to the new space
    // so the first references might change
    // the reference to the first element would be pointing to deallocated memory
    // b.push(6);
    println!("The first element is: {}", first);

    for i in &mut b {
        println!("{}", *i + 50);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // the type of all elements is SpreadsheetCell
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("hello")),
    ];
}
