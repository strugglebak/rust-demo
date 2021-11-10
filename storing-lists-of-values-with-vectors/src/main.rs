fn main() {
    // v value changed, so we need to make it mutable
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // vec! is a macro
    let a = vec![1, 2, 3];
}
