pub fn run(){
    //Print to console
    println!("Hello from print rs file");

    //BAsic formating
    println!("Number: {} {}",1, "brad");
    
    //Positional argguments
    println!("{0} is from {1} and {0} lines to {2}","brad", "mass","code");

    //Named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "basebasll");

    //Placeholder traits
    println!("binaru: {:b} hex: {:x} octal: {:o}", 10,10,10);

    //Placeholder for  debug trait , touple
    println!("{:?} ", (12,true, "Hello"));

    //basoc math
    println!("10 + 10 = {}", 10 + 10 );










}