use std::thread;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}
impl Worker {
  fn new(id: usize) -> Worker {
    let thread = thread::spawn(|| {});
    Worker {
      id,
      thread,
    }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
};

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
      workers.push(Worker::new(id));
    }
    ThreadPool { workers }
  }

  pub fn execute<F>(&self, f: F)
  where
    // 1. `FnOnce`: the thread for running a request will
    //    only execute that request’s closure one time
    // 2. we need `Send` to transfer the closure from one thread to another
    // 3. `'static`
    F: FnOnce() + Send + 'static
  {

  }
}
