fn main() {

    let mut vec = vec![1,3,5,7,9];
    
    let mut result: Vec<i16> = vec.iter().map(|x| x * 10).collect();

    let mut filtered_result: Vec<i16> = result.into_iter().map(|x| x * 10).collect();
    
    // println!("{:?}", result);
    println!("{:?}", filtered_result);
}
