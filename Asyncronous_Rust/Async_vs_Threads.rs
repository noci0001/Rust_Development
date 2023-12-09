//LEARN HOW TO USE THE ASYNCRONOUS FEATURES
// TO GET THE BEST OUT OF OUR HARDWARE
// ENDGOAL: BUILD A CLIENT SERVER-SIDE CHAT APPLICATION!

use async_std::{fs::File, io, prelude::*, task};

// trait Future {
//     type Output;
//     fn poll(self: Pin<&mut self>, cx: &mut Context) -> Poll<Self::Output>;
    
//     //poll::ready
//     //poll::pending
// }


async fn read_file(path: &str) -> io:Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file::read_to_string(&mut contents).await?;
    Ok(contents)
}

//THREADS 
//   -> small number of tasks (CUP and memory overhead)
//   -> we can use asyncronous functions
//ASYNC
//  -> much higher amount of tasks than system threads (low CPU and Memory overhead)
int main () {
    let task = task::spawn(async {
       let result = read_file("read.txt").await;
       match result {
           Ok(k) => println!("{}", k),
           Err(e) => println!("Error reading from file: {}", e),
       }
    });
    
    
    println!("Task has started!");
    task::block_on(task);
    println!("Stopped task!");
}
