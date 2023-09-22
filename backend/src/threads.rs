use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};

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

        (0..size).for_each(|i| workers.push(Worker::new(i, receiver.clone())));
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
            if let Some(thread) = w.0.take() {
                thread.join().unwrap();
            }
        });
    }
}

struct Worker(Option<JoinHandle<()>>);

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::Builder::new()
            .name(format!("worker-#{}", id))
            .spawn(move || {
                loop {
                    let lock = receiver.lock().unwrap();
                    let job = lock.recv().unwrap();
                    drop(lock);
                    job();
                }
            }).unwrap();

        Self(Some(thread))
    }
}
