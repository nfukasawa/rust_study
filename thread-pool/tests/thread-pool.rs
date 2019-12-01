use std::sync::Arc;
use std::sync::{Condvar, Mutex};
use threadpool::ThreadPool;

#[derive(Clone)]
struct CounterTask {
    state: Arc<(Mutex<usize>, Condvar)>,
}

impl CounterTask {
    pub fn new() -> Self {
        Self {
            state: Arc::new((Mutex::new(0), Condvar::new())),
        }
    }
    pub fn run(&self) {
        let (val, condvar) = &*self.state;
        let mut val = val.lock().unwrap();
        *val += 1;
        condvar.notify_all();
    }
    pub fn wait_for_run_count(&self, count: usize) -> usize {
        let (val, condvar) = &*self.state;
        let mut val = val.lock().unwrap();
        if *val < count {
            val = condvar.wait(val).unwrap();
        }
        *val
    }
}

#[derive(Clone)]
struct LatchTask {
    state: Arc<(Mutex<(usize, usize)>, Condvar)>,
}

impl LatchTask {
    pub fn new(count: usize) -> Self {
        Self {
            state: Arc::new((Mutex::new((count, 0)), Condvar::new())),
        }
    }
    pub fn run(&self) {
        let (val, condvar) = &*self.state;
        let mut val = val.lock().unwrap();
        val.1 += 1;
        condvar.notify_all();
        while val.1 < val.0 {
            val = condvar.wait(val).unwrap();
        }
    }
    pub fn wait_for_latch_count(&self) {
        let (val, condvar) = &*self.state;
        let mut val = val.lock().unwrap();
        while val.1 < val.0 {
            val = condvar.wait(val).unwrap();
        }
    }
}

#[test]
#[should_panic]
fn test_constructor_illegal_argument_first() {
    ThreadPool::new(0, 1);
}

#[test]
#[should_panic]
fn test_constructor_illegal_argument_second() {
    ThreadPool::new(1, 0);
}

#[test]
fn test_simple_dispatch() {
    let pool = ThreadPool::new(1, 1);
    let task = CounterTask::new();
    let task2 = task.clone();
    pool.dispatch(move || task2.run()).unwrap();
    task.wait_for_run_count(1);
}

#[test]
fn test_simple_repeated_dispatch() {
    let pool = ThreadPool::new(1, 1);
    let task = CounterTask::new();
    for _ in 0..10 {
        let task = task.clone();
        pool.dispatch(move || task.run()).unwrap();
    }
    task.wait_for_run_count(10);
}

#[test]
fn test_complex_repeated_dispatch() {
    let pool = ThreadPool::new(10, 10);
    let task = CounterTask::new();
    for _ in 0..1000 {
        let task = task.clone();
        pool.dispatch(move || task.run()).unwrap();
    }
    task.wait_for_run_count(1000);
}

#[test]
fn test_complex_repeated_dispatch2() {
    let pool = ThreadPool::new(10, 10);
    let mut tasks = Vec::new();
    for _ in 0..10 {
        tasks.push(CounterTask::new());
    }
    for _ in 0..100 {
        tasks.iter().for_each(|task| {
            let task = task.clone();
            pool.dispatch(move || task.run()).unwrap();
        })
    }
    tasks.iter().for_each(|task| {
        task.wait_for_run_count(100);
    })
}

#[test]
fn test_latch_simple_dispatch() {
    let number_of_threads = 10;
    let pool = ThreadPool::new(10, number_of_threads);
    let task = LatchTask::new(number_of_threads);
    for _ in 0..number_of_threads {
        let task = task.clone();
        pool.dispatch(move || task.run()).unwrap();
    }
    task.wait_for_latch_count();
}

#[test]
fn test_latch_complex_dispatch() {
    let number_of_threads = 10;
    let pool = ThreadPool::new(10, number_of_threads);
    let mut tasks = Vec::new();
    for _ in 0..10 {
        tasks.push(LatchTask::new(number_of_threads));
    }
    tasks.iter().for_each(|task| {
        for _ in 0..number_of_threads {
            let task = task.clone();
            pool.dispatch(move || task.run()).unwrap();
        }
    });
    tasks.iter().for_each(|task| {
        task.wait_for_latch_count();
    });
}