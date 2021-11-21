use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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

    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // error:
        // The send function takes ownership of its parameter
        // and when the value is moved, the receiver takes ownership of it
        // println!("val is {}", val);
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the2"),
            String::from("thread2"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv will block the main thread’s execution
    // and wait until a value is sent down the channel
    // try_recv doesn’t block
    // both return Result<T, E>
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }
}
