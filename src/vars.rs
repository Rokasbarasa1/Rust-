//Variables hold primitive data or references to data
//Variables are imutable by default
//Rust is a block scoped language


pub fn run(){
    let name = "";
    let mut age = 37;
    println!("MY name is {} and i am {}", name, age);

    age = 38;

    println!("MY name is {} and i am {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assig multiple variables
    let (my_name, my_age) = ("brad", 37);
    println!("{} is {}", my_name, my_age);
}