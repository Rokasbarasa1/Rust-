// Vectors - Resizable array

use std::mem;
pub fn run(){
    //FIxed sized arrays must be exact lenths as well as the data type
    let mut numbers: Vec<i32> = vec! [1,2,3,4];

    //Re-assign vlaue
    numbers[2] = 20;

    //Add onto vector

    numbers.push(5);
    numbers.push(6);

    //Pop last value
    numbers.pop();

    println!("{:?}", numbers);

    //Get single val
    println!("Single value {}", numbers[0]);

    //Get array length 
    println!("Vector lentht {}", numbers.len());

    //Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice, get some of them
    let slice: &[i32] = &numbers[0..2];

    println!("Slice {:?}", slice);


    //Loop trough vecor values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //Loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    

    println!("Mumbers vec {:?}", numbers)
}