use std::thread;
use std::rc::Rc;
// MULTIPRODUCER SINGLE CONSUMER!!!
use std::sync::mpsc;

use std::sync::{Arc, Mutex};

fn main() {
    let handle = std::thread::spawn(move || {
        println!("Hello from thread!");
    });
    
    handle.join().unwrap();
    
    println!("Hello from main");
    
    let v = vec![1,2,3];
    
    let handle = std::thread::spawn(move || {
      println!("{:?}", v); 
    });
    
    let mut thread_handles = Vec::new();
    
    for e in v {
        thread_handles.push(thread::spawn(move || println!("Thread {}", e)));
    }

    println!("Main thread!");
    
    for handle in thread_handles {
        handle.join().unwrap();
    }
    
    // ------------------------------------------
    
    let (transmitter, receiver) = mpsc::sync_channel(1000);
    let tx = transmitter.clone();
    
    let val = String::from("Transmitting!");
    std::thread::spawn(move || {
        transmitter.send(val).unwrap();
    })

    let msg = receiver.recv().unwrap();
    prtintln!("{}", msg);
    
    std::thread::spawn(move || {
        let vec = vec![ String::from("Transmitting"),
                        String::from("From"),
                        String::from("Original"),
                        ];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });
    
    std::thread::spawn(move || {
        let vec = vec![ String::from("Clone"),
                        String::from("Is"),
                        String::from("Transmitting"),
                        ];
        for val in vec {
            tx.send(val).unwrap();
        }
    });
    
    
    for rec in receiver {
        println!("{}", rec);
    }
    let rc1 = Arc::new(String::from("Test"));
    let rc2 = rc1.clone();
    
    std::thread::spawn(move || {
        rc2;
    });
    
    //Create atomic reference counter 
    // and vector that stores thread handles
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    //create a loop from 0 to 7
    // create a cloned reference of counter
    // create a thread that owns the variable 
    // inside, we lock the variable and we add 1;
    // we finally push the threadhandle inside the vector for then joining them all
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // let mut num2 = counter.lock().unwrap();
            *num += 1;
            println!("{}", num);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("{}", counter.lock().unwrap());
}
