use std::{process, thread};

pub struct Callable<T: FnOnce()> {
    on_panic_unwind: Option<T>,
}

impl<T: FnOnce()> Callable<T> {
    pub fn new(on_panic_unwind: T) -> Callable<T> {
        Callable {
            on_panic_unwind: Some(on_panic_unwind),
        }
    }
}

impl<T: FnOnce()> Drop for Callable<T> {
    fn drop(&mut self) {
        if thread::panicking() {
            self.on_panic_unwind
                .take()
                .expect("on_panic_unwind should not be None")();
        }
    }
}

pub fn abort_process() {
    process::abort();
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        cell::OnceCell,
        panic::{self, AssertUnwindSafe},
        rc::Rc,
    };

    #[test]
    fn new_constructs_a_proper_callable() {
        let c = Callable::new(abort_process);

        assert!(
            c.on_panic_unwind.is_some(),
            "on_panic_unwind was not assigned"
        );
        assert_eq!(
            c.on_panic_unwind.unwrap() as usize,
            abort_process as usize,
            "on_panic_unwind does not have the proper value"
        );
    }

    #[test]
    fn calls_on_panic_unwind_when_thread_panics() {
        let was_called = Rc::new(OnceCell::new());
        let was_called_clone = AssertUnwindSafe(Rc::clone(&was_called));

        let on_panic_unwind = || {
            was_called_clone.get_or_init(|| true);
        };

        let catch_result = panic::catch_unwind(move || {
            let _c = Callable::new(on_panic_unwind);
            panic!();
        });

        assert!(catch_result.is_err(), "test did not panic");
        assert!(
            was_called.get().unwrap_or(&false),
            "on_panic_unwind was not called"
        );
    }

    #[test]
    fn does_not_call_on_panic_unwind_when_thread_does_not_panic() {
        let was_called = Rc::new(OnceCell::new());
        let was_called_clone = AssertUnwindSafe(Rc::clone(&was_called));

        let on_panic_unwind = || {
            was_called_clone.get_or_init(|| true);
        };

        let catch_result = panic::catch_unwind(move || {
            let _c = Callable::new(on_panic_unwind);
        });

        assert!(catch_result.is_ok(), "test panicked");
        assert!(was_called.get().is_none(), "on_panic_unwind was called");
    }
}
