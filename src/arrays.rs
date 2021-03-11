// Arrays - Fixed list where elements are the same data types

use std::mem;
pub fn run(){
    //FIxed sized arrays must be exact lenths as well as the data type
    let mut numbers: [i32; 4] = [1,2,3,4];

    //Re-assign vlaue
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single val
    println!("Single value {}", numbers[0]);

    //Get array length 
    println!("array lentht {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice, get some of them
    let slice: &[i32] = &numbers[0..2];

    println!("Slice {:?}", slice);
}