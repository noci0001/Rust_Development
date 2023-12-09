use macro::debug_print;

// Create a macro called "op" that takes three arguments. 
// All three arguments will be integers, but the third argument 
// is going to be how we tell the macro what operation to perform.

// Operations
// 1 is add, 2 is subtract, 3 is multiply, 4 is divide, and 5 is mod.

//     println!("{}", op!(5,2,1)); //should print 7
//     println!("{}", op!(5,2,2)); //should print 3
//     println!("{}", op!(5,2,3)); //should print 10
//     println!("{}", op!(5,2,4)); //should print 2
//     println!("{}", op!(5,2,5)); //should print 1
//     println!("{}", op!(5,2,6)); //should print -1

macro_rules! op {
    ( $x:expr, $y:expr, 1 ) => {
        $x + $y
    };
    ( $x:expr, $y:expr, 2 ) => {
        $x - $y
    };
    ( $x:expr, $y:expr, 3 ) => {
        $x * $y
    };
    ( $x:expr, $y:expr, 4 ) => {
        $x / $y
    };
    ( $x:expr, $y:expr, 5 ) => {
        $x % $y
    };
    ( $x:expr, $y:expr, 6 ) => {
        $x * -1
    };
}

fn main() {
    println!("{}", op!(5, 2, 1)); // should print 7
    println!("{}", op!(5, 2, 2)); // should print 3
    println!("{}", op!(5, 2, 3)); // should print 10
    println!("{}", op!(5, 2, 4)); // should print 2
    println!("{}", op!(5, 2, 5)); // should print 1
    println!("{}", op!(5, 2, 6)); // should print -1
}
