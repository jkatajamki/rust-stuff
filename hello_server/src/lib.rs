use std::{error::Error, fmt};
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

mod fn_box;

type Job = Box<dyn fn_box::FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
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

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            let cloned_receiver = Arc::clone(&receiver);

            workers.push(Worker::new(id, cloned_receiver));
        }

        ThreadPool {
            workers,
            sender,
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
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
