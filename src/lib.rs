use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>, // ThreadPool holds sending end of channel
}

trait FnBox {
    fn call_box(self: Box<self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;


impl ThreadPool{

    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if size is 0
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        // create new channel
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // pass receiving end of channel to worker
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    /// send job from ThreadPool to worker instances
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();

    }
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,

}

//  what the fuck is our closure here?!?!!! receiver

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move ||{
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got job; executing ...", id);

                job.call_box();

            }
        });

        Worker {
            id,
            thread,
        }
    }

}
