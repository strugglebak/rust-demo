use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread will wait for
    // the spawned thread to finish and then run its for loop
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // make sure the spawned thread finishes before main exits
    // handle.join().unwrap();

    let v = vec![1, 2, 3];
    // error:
    // thread::spawn runs this closure in a new thread
    // we should be able to access v inside that new thread

    // let handle2 = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // force the closure to take ownership of the values
    // it’s using rather than allowing Rust to infer that
    // it should borrow the values
    let handle2 = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });
    // error:
    // adding move to the closure
    // we could no longer call drop on it in the main thread
    // drop(v);
    handle2.join().unwrap();
}
