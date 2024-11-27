use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam::atomic::AtomicCell;

// Traditional lock-based approach
fn traditional_lock_example() {
    let shared_value = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let shared_value_clone = Arc::clone(&shared_value);
        let handle = thread::spawn(move || {
            for _ in 0..10000 {
                let mut value = shared_value_clone.lock().unwrap();
                *value += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Traditional lock-based approach result: {}", *shared_value.lock().unwrap());
}

// Lock-free data structure approach using AtomicCell from crossbeam
fn lock_free_example() {
    let shared_value = Arc::new(AtomicCell::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let shared_value_clone = Arc::clone(&shared_value);
        let handle = thread::spawn(move || {
            for _ in 0..10000 {
                // Simulate fetch_add using AtomicCell's load and store
                loop {
                    let current = shared_value_clone.load();
                    let new = current + 1;
                    if shared_value_clone.compare_exchange(current, new).is_ok() {
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Lock-free data structure approach result: {}", shared_value.load());
}

fn main() {
    traditional_lock_example();
    lock_free_example();
}
