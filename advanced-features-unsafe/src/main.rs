// unsafe superpowers
// can’t in safe Rust

// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of unions S

extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// By using unsafe impl, we’re promising that
// we’ll uphold the invariants that the compiler can’t verify.
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// Accessing union fields is unsafe because Rust can’t guarantee
// the type of the data currently being stored in the union instance

fn main() {
    // create an immutable and a mutable raw pointer from references
    let mut num = 5;
    // potentially creating a data race
    let r1 = &num as *const i32;
    let r2 = &mut num as *const i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    add_to_count(3);

    // we can’t dereference raw pointers and read the data being pointed to
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // segmentation fault
        // println!("r is: {}", *r);
        // we don’t need to add another unsafe block
        dangerous();
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // as_mut_ptr returns a raw pointer with the type *mut i32
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
    // (&mut slice[..mid], &mut slice[mid..])
    // from_raw_parts_mut is unsafe because it takes a raw pointer
    // and must trust that this pointer is valid
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            // The add method on raw pointers is also unsafe
            // because it must trust that the offset location is also a valid pointer
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }

}
