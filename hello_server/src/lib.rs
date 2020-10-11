use std::{error::Error, fmt};
use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

#[derive(Debug)]
pub struct PoolCreationError;

impl Error for PoolCreationError {}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to create thread pool.")
    }
}

impl ThreadPool {
    fn get_threads(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers
        }
    }

    /// Create a new ThreadPool,
    ///
    /// The size is the number of threads in the pool.
    /// Size must be greater than 0, or `new` will return a PoolCreationError
    ///
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError),
            _ => Ok(ThreadPool::get_threads(size)),
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {

    }
}
