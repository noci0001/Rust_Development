
//    __________________
//   |                  |           EXPERIMENTAL
//   |  BIG O NOTATION  | TWO WAYS
//   |__________________|           THEORETICAL
//
// SHOWS HOW EFFICIENT AN ALGORITHM IS
// IN THE WORST CASE SCENARIO
//
// IT IS BASED ON TWO FACTORS:
//
// (T) TIME => HOW MUCH IT TAKES TO RUN
//
// (N) SPACE => HOW MUCH MEMORY IT TAKES TO RUN
//
//
//
// THE FUNCTION IS -> T(N)
//
// T  |           x                    N is 1 (Linear expression)
// i  |        x                       |
// m  |     x               T(N) = a(n"1)+b
// e  |  x                  IDEAL: LESS TIME AND LESS MEMORY
//    |x______________      NOT IDEAL: MORE TIME AND MORE MEMORY
//    0  1  2  3  4  5
//       n - input size
//
//
//____________________________________________________________________
//
//
//
// T  |     X                          N is 2 (quadratic expression)
// i  |    X                           |
// m  |   X                T(N) = a(n"2)+bn+c 
// e  |  X                  
//    |x______________ 
//    0  1  2  3  4  5
//       n - input size
//___________________________________________________________________

fn main() {
    
    let mut vec = Vec::new();

    vec.push(5);
    vec.push(3);
    vec.push(6);
    vec.push(1);
    vec.push(8);

    for x in vec {
        println!("{}", x);
    }

}
