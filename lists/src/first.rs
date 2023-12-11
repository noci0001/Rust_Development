use std::mem;

//A mutable reference represents temporary exclusive 
// access to a value that you don't own

// pub indicates that we want to use this function outside
// of this module
pub struct List {
    head: Link
}

//impl => to associate actual code with a type
// :: => name spacing operator
impl List{

    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty), 
        });

        self.head = Link::More(new_node);
    }
}


enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

//fn main() {
//    println!("{}", "Hello World!");
//}
