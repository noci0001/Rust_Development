use std::rc::Rc;
use std::cell::RefCell;

struct Flagger {
    is_true: Rc<RefCell<bool>>,
}

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
    
    println!("{} {} {}", s1.contains("Point"), s2, s3.contains("ter"));

    let flag = Flagger{is_true: Rc::new(RefCell::new(true))};
    
    let reference = Rc::new(flag.is_true.clone());
    println!("{:?}", reference);
    
    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false;
    
    println!("{}", mut_ref);
}
