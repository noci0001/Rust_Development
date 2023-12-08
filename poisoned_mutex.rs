use std::thread;
use std::rc::Rc;
// MULTIPRODUCER SINGLE CONSUMER!!!
use std::sync::mpsc;

use std::sync::{Arc, Mutex};

fn main() {
    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);
    
    let _ = std::thread::spawn(move || -> () {
        let _guard = lock2.lock().unwrap(); // we acquire the lock here
        panic!(); //mutex is now poisoned
    }).join();
    
    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    
    *guard += 1;
    println!("{:?}", guard);
}
