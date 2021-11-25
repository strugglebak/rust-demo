// unsafe superpowers
// can’t in safe Rust

// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of unions S

fn main() {
    // create an immutable and a mutable raw pointer from references
    let mut num = 5;
    // potentially creating a data race
    let r1 = &num as *const i32;
    let r2 = &mut num as *const i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    // we can’t dereference raw pointers and read the data being pointed to
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // segmentation fault
        // println!("r is: {}", *r);
        // we don’t need to add another unsafe block
        dangerous();
    }
}

unsafe fn dangerous() {}
