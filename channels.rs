// use std::thread;
// MULTIPRODUCER SINGLE CONSUMER!!!
use std::sync::mpsc;

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
}
