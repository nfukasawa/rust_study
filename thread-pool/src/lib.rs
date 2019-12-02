use std::error::Error;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub type ThreadPoolError = Box<dyn Error>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::SyncSender<Msg>,
}

impl ThreadPool {
    pub fn new(queue_size: usize, number_of_threads: usize) -> Self {
        assert_ne!(queue_size, 0);
        assert_ne!(number_of_threads, 0);

        let mut workers = Vec::with_capacity(number_of_threads);
        let (sender, receiver) = mpsc::sync_channel(queue_size);
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..number_of_threads {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }
        Self { workers, sender }
    }

    pub fn dispatch<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        match self.sender.send(Msg::Dispatch(job)) {
            Err(err) => Err(Box::new(err)),
            _ => Ok(()),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Msg::Stop).unwrap();
        }
        while let Some(worker) = self.workers.pop() {
            worker.join();
        }
    }
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Msg>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv().unwrap();
            match msg {
                Msg::Dispatch(job) => job.call_box(),
                Msg::Stop => break,
            }
        });
        Self { thread }
    }

    fn join(self) {
        self.thread.join().unwrap();
    }
}

enum Msg {
    Dispatch(Job),
    Stop,
}

type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
