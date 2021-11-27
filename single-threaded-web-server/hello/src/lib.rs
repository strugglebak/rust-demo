use std::thread;
use std::sync::{mpsc, Arc, Mutex};

struct Job;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}
impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(|| {
      receiver;
    });
    Worker {
      id,
      thread,
    }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
};

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

  }
}
