use crate::{
    panic_unwind::Callable,
    types_traits::{FnOnceSend, Job},
};

use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Job>>>,
        on_panic_unwind: impl FnOnceSend,
    ) -> Worker {
        let thread = thread::spawn(move || {
            let _on_panic_unwind = Callable::new(on_panic_unwind);

            loop {
                let message = receiver
                    .lock()
                    .expect("receiver lock should not be poisoned")
                    .recv();

                if let Ok(job) = message {
                    println!("Worker {id} got a job. Executing.");
                    job();
                } else {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }

    #[cfg(test)] // currently used only by tests
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        println!("Shutting down worker {}.", self.id);

        #[allow(unused_variables)]
        let join_result = self
            .thread
            .take()
            .expect("thread should not be None")
            .join();

        #[cfg(not(test))] // when testing, we sometimes want to run code that panics
        join_result.expect("worker thread should not panic");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::panic_unwind;

    use std::sync::{mpsc, OnceLock};

    #[test]
    fn new_creates_a_proper_worker() {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let worker = Worker::new(15, receiver, panic_unwind::abort_process);
        assert_eq!(worker.id(), 15, "worker id is different");
        assert!(
            worker.thread.is_some(),
            "no worker thread was assigned to the worker"
        );

        drop(sender);
    }

    #[test]
    fn executes_code_on_another_thread() {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let worker = Worker::new(0, receiver, panic_unwind::abort_process);

        let this_thread_id = thread::current().id();
        let worker_thread_id = Arc::new(OnceLock::new());
        let worker_thread_id_clone = Arc::clone(&worker_thread_id);

        let get_worker_thread_id = Box::new(move || {
            worker_thread_id_clone.get_or_init(|| thread::current().id());
        });

        assert!(
            sender.send(get_worker_thread_id).is_ok(),
            "failed to get the worker thread id"
        );

        drop(sender);
        drop(worker);

        assert!(
            worker_thread_id.get().is_some(),
            "no worker thread id was assigned"
        );

        assert_ne!(
            *worker_thread_id.get().unwrap(),
            this_thread_id,
            "the thread id of the worker thread was the same as the one for the main thread"
        );
    }

    #[test]
    fn calls_on_panic_unwind_when_thread_panics() {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let was_called = Arc::new(OnceLock::new());
        let was_called_clone = Arc::clone(&was_called);

        let on_panic_unwind = move || {
            was_called_clone.get_or_init(|| true);
        };

        let worker = Worker::new(0, receiver, on_panic_unwind);

        let panic = Box::new(|| {
            panic!();
        });

        assert!(sender.send(panic).is_ok(), "failed to send a panic message");

        drop(sender);
        drop(worker);

        assert!(
            was_called.get().unwrap_or(&false),
            "on_panic_unwind was not called"
        );
    }
}
