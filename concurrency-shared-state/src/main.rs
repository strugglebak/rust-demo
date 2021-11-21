// use std::sync::Mutex;
// atomically reference counted
// Arc<T> is a type like Rc<T> that is safe to use in concurrent situations
use std::sync::{Arc, Mutex};
use std::thread;
// use std::rc::Rc;

fn main() {
    // let m = Mutex::new(5);
    // {
    //     // lock returns MutexGuard
    //     // which implements Deref
    //     // also has a Drop implementation that releases
    //     // the lock automatically when a MutexGuard goes out of scope
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m);

    // let counter = Mutex::new(0);
    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // error:
        // the counter value was moved in the previous iteration of the loop
        // So Rust is telling us that we can’t move the ownership
        // of lock counter into multiple threads
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);
        // error:
        // Rc<T> is not safe to share across threads
        // But it doesn’t use any concurrency primitives
        // to make sure that changes to the count can’t be interrupted by another thread
        let handle = thread::spawn(move || {
            let mut num  = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
