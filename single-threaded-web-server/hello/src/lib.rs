use std::thread;

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
};

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    let mut threads = Vec::with_capacity(size);
    for _ in 0..size {
      // create some threads and store them in the vector
    }
    ThreadPool { threads }
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
