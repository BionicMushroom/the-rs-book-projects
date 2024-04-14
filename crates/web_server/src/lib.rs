//! Contains an implementation of a [`ThreadPool`]
//! that can be used for serving web requests.

mod panic_unwind;
mod types_traits;
mod worker;

use std::{
    num::NonZeroUsize,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
};

use types_traits::Job;
use worker::Worker;

/// Manages a pool with a specified number of threads. See
/// [`ThreadPool::build`] for an example.
#[derive(Debug)]
pub struct ThreadPool {
    sender: Sender<Job>,

    #[allow(dead_code)] // used for Drop side-effect and in tests
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// The `size` parameter specifies the number of threads in the pool.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{num::NonZeroUsize, thread};
    /// use web_server::ThreadPool;
    ///
    /// println!("Main thread has id {:?}.", thread::current().id());
    ///
    /// let pool = ThreadPool::build(NonZeroUsize::new(1usize).unwrap());
    ///
    /// pool.execute(|| {
    ///     println!("Thread pool thread has id {:?}.", thread::current().id());
    /// });
    /// ```
    pub fn build(size: NonZeroUsize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size.get());

        for id in 0..size.get() {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                panic_unwind::abort_process,
            ));
        }

        ThreadPool { sender, workers }
    }

    /// Runs a given piece of code on a thread from the [`ThreadPool`].
    ///
    /// # Panics
    ///
    /// If the piece of code given to `execute` panics, the process is aborted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{num::NonZeroUsize, time::Duration, thread};
    /// use web_server::ThreadPool;
    ///
    /// let num_threads = NonZeroUsize::new(10usize).unwrap();
    /// let tp = ThreadPool::build(num_threads);
    ///
    /// for _ in 0..num_threads.get() {
    ///     tp.execute(|| {
    ///         println!("Thread id {:?} beginning work.", thread::current().id());
    ///         // simulate some long-running code
    ///         thread::sleep(Duration::from_secs(10));
    ///         println!("Thread id {:?} finished work.", thread::current().id());
    ///     });
    /// }
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Err(e) = self.sender.send(job) {
            panic!("send should not fail: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        sync::{Barrier, OnceLock},
        thread::{self, ThreadId},
    };

    #[test]
    fn build_constructs_a_proper_thread_pool() {
        let num_threads = NonZeroUsize::new(10usize).unwrap();
        let tp = ThreadPool::build(num_threads);

        assert_eq!(
            tp.workers.len(),
            num_threads.get(),
            "not enough workers were created"
        );

        for (index, worker) in tp.workers.iter().enumerate() {
            assert_eq!(
                index,
                worker.id(),
                "worker at index {index} did not have the correct id"
            );
        }
    }

    #[test]
    fn executes_code_on_separate_threads() {
        let num_threads = NonZeroUsize::new(2usize).unwrap();
        let tp = ThreadPool::build(num_threads);

        let barrier = Arc::new(Barrier::new(num_threads.get()));
        let this_thread_id = thread::current().id();

        let get_worker_thread_id = |thread_id: Arc<OnceLock<ThreadId>>, barrier: Arc<Barrier>| {
            thread_id.get_or_init(|| thread::current().id());
            barrier.wait();
        };

        let first_worker_thread_id = Arc::new(OnceLock::new());
        let worker_thread_id_clone = Arc::clone(&first_worker_thread_id);
        let barrier_clone = Arc::clone(&barrier);

        tp.execute(move || {
            get_worker_thread_id(worker_thread_id_clone, barrier_clone);
        });

        let second_worker_thread_id = Arc::new(OnceLock::new());
        let worker_thread_id_clone = Arc::clone(&second_worker_thread_id);
        let barrier_clone = Arc::clone(&barrier);

        tp.execute(move || {
            get_worker_thread_id(worker_thread_id_clone, barrier_clone);
        });

        drop(tp);

        assert!(
            first_worker_thread_id.get().is_some(),
            "no first worker thread id was assigned"
        );

        let first_worker_thread_id = *first_worker_thread_id.get().unwrap();

        assert_ne!(
            first_worker_thread_id, this_thread_id,
            "the thread id of the first worker thread was the same as the main thread"
        );

        assert!(
            second_worker_thread_id.get().is_some(),
            "no second worker thread id was assigned"
        );

        let second_worker_thread_id = *second_worker_thread_id.get().unwrap();

        assert_ne!(
            second_worker_thread_id, this_thread_id,
            "the thread id of the second worker thread was the same as the one for the main thread"
        );

        assert_ne!(
            first_worker_thread_id, second_worker_thread_id,
            "the thread id of the first worker thread was the same as the one for the second worker thread"
        );
    }
}
