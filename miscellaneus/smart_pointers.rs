// Create a variable on the stack and a variable on the heap. Multiply their values and print out the results.

use std::rc::Rc;

fn main() {
     let mut stack_var: i32 = 8;
     let mut heap_var: Box<i32> = Box::new(2);
    
     println!("MULTIPLIED VALUES: {}", stack_var * *heap_var);
 }


// Create a variable that holds a String. Then create a reference counting smart pointer that 
// contains the String. Print out how many references the smart pointer has. 
// Now inside the code block create another 
//reference counting smart pointer that points to our first smart pointer. 
//Print out how many references each smart pointer has.
fn main() {
    let my_string: String = String::from("This is a string");
    let reference = Rc::new(my_string);
    
    println!("Reference Count 1: {}", Rc::strong_count(&reference));
    
    let reference2 = Rc::clone(&reference);
    println!("Reference Count 1: {}", Rc::strong_count(&reference));
    println!("Reference Count 2: {}", Rc::strong_count(&reference2));
}
