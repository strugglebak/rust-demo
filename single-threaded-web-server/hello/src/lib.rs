use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}
impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    // we need the closure to loop forever
    // asking the receiving end of the channel for a job
    // and running the job when it gets one
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();
      println!("Worker {} got a job; executing.", id);
      job();
    });

    // ↓
    // This code compiles and runs but doesn’t result in the desired threading behavior
    // while let (and if let and match) does not drop temporary values until the end of the associated block
    // but let doesn't
    // so the lock remains held for the duration of the call to job(),
    // meaning other workers cannot receive jobs.
    // ↓

    // let thread = thread::spawn(move|| {
    //   while let Ok(job) = receiver.lock().unwrap().recv() {
    //     println!("Worker {} got a job; executing.", id);
    //     job();
    //   }
    // });
    Worker {
      id,
      thread,
    }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    // the channel implementation that Rust provides is multiple producer, single consumer
    // This means we can’t just clone the consuming end of the channel to fix this code
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    // 1. `FnOnce`: the thread for running a request will
    //    only execute that request’s closure one time
    // 2. we need `Send` to transfer the closure from one thread to another
    // 3. `'static` we don’t know how long the thread will take to execute
    F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}
