pub struct ThreadPool;
impl ThreadPool {
  pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    assert!(size > 0);
    ThreadPool
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
