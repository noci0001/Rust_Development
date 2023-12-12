use std::mem;



// &mut => A mutable reference represents temporary exclusive 
// access to a value that you don't own

// pub indicates that we want to use this function outside
// of this module
pub struct List {
    head: Link
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

//impl => to associate actual code with a type
// :: => name spacing operator
impl List {

    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None), 
        });

        self.head = Link::More(new_node);
    }

    // Option is an enum that represents a value that may exist.
    // It can either be Some(T) or None
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    //unimplemented!()
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    // The test should be compiled only if tests are run
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

//fn main() {
//    println!("{}", "Hello World!");
//}
