// use rand::seq::SliceRandom;
// use rand::thread_rng;
use std::collections::{binary_heap, BinaryHeap};

fn main() {
    // //create empty vec
    // let mut nums:Vec<i32> = vec![];
    // //fill it
    // nums.push(1);
    // nums.push(2);
    // nums.push(3);
    // //register popped var
    // let pop = nums.pop();
    // //print that
    // println!("{:?}", pop);

    // let two = nums[1]; //copy
    // println!("{}", two);

    // let one = nums.first(); //None if empty or Some<T> if not empty

    // println!("{:?}", one);

    // println!("Len is: {}", nums.len());
    // println!("Len is empty: {}", nums.is_empty());

    // nums.insert(0, 10);
    // nums.insert(3, 12);
    // nums.insert(2, 25);

    // nums.remove(3);

    // nums.sort();
    // println!("{:?}", nums);
    // nums.reverse();
    // println!("REVERSE: {:?}", nums);

    // nums.shuffle(&mut thread_rng());
    // println!("SHUFFLED -> {:?}", nums);

    //BINARY HEAP -> collection whose elements are organized.
    // the greatest value always is at the front 

    let mut bheap = BinaryHeap::new();

    bheap.push(1);
    bheap.push(18);
    bheap.push(20);
    bheap.push(5);

    println!("{:?}", bheap);
}