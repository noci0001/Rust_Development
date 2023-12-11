// pub indicates that we want to use this function outside
// of this module
pub struct List {
    head: Link
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
