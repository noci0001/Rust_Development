use std::rc::Rc;

fn main() {
    //smart pointer
    let t = (12, "eggs"); //created on the stack
    let b = Box::new(t); //created on the heap but b was stored in the stack
    
    println!("{:?}", b);
    
    let x = 5;
    let y = &x;
    
    assert_eq!(5, x);
    
    assert_eq!(5, *y);
    
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    
    assert_eq!(5, *y);

    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();
    
    println!("{} {} {}", s1.contains("Point"), s2, s3.contain("ter"));
}
