use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};
use std::panic;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self { 
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        (0..size).for_each(|i| workers.push(Worker::new(i, Arc::clone(&receiver))));
        Self { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        
        self.workers.iter_mut().for_each(|w| {
            println!("shutting down `worker-#{}`", w.1);

            if let Some(thread) = w.0.take() {
                let _ = thread.join();
            }
        });
    }
}

struct Worker(Option<JoinHandle<()>>, usize);

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::Builder::new()
            .name(format!("worker-#{}", id))
            .spawn(move || {
                loop {
                    match receiver.lock().unwrap().recv() {
                        Ok(job) => if let Err(e) = panic::catch_unwind(panic::AssertUnwindSafe(job)) {
                            println!("{:?}", e);
                        },
                        Err(_) => {
                            println!("thread '{}' shutting down", id);
                            break
                        },
                    };
                }
            }).unwrap();

        Self(Some(thread), id)
    }
}
