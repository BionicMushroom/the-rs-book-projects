use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));

    // we can comment the line below to avoid the deadlock
    let _lock = m.lock().unwrap();

    let worker_m = Arc::clone(&m);
    let handle = thread::spawn(move || {
        println!("In the worker thread, before taking the lock.");
        let _lock = worker_m.lock().unwrap();
        println!("In the worker thread, after taking the lock.");
    });

    println!("In the main thread, before waiting for the worker.");
    handle.join().unwrap();
    println!("In the main thread, after waiting for the worker.");
}
