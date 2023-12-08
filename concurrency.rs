use std::thread;

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
}
