//by default reyon uses the same amount of threads 
// as the number of logical CPUs available (with Hyper-Threading turned on)
//if hyperthreading is turned off, it will be based on the number of physical CPUs

use rayon::prelude::*;
use num::{BigUint, One};
// use std::time::Instant;

// if we use largest Uint that rust provide our factorial 
// would arrive only to 35, so we use BigUint to use much larger values
fn factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one()
    } else {
        //we go from 1 to num (included)
        //
        (1..=num).map(BigUint::from).reduce(|acc, x| acc * x).unwrap()
    }
}

fn multi_fact(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one()
    } else {
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one() |acc, x| acc * x)
    }
}

//Program that computes the factorial of a number
// COMPARISON BETWEEN SINGLE THREADING and MULTITHREADING
fn main() {
    let now = Instant::now();
    factorial(50000);
    pritnln!("{:.2?}", now.elapsed()); //ELAPSING TIME: 4.92s
    
    let now = Instant::now();
    multi_fact(50000);
    pritnln!("{:.2?}", now.elapsed()); //ELAPSING TIME: 169.50ms
}
