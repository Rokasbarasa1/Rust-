


pub fn run(){
    greeting("hello,","jane");

    // bubd function values t variables
    let get_sum = add(5,5);
    println!("{}", get_sum);

    //Clopsure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3,3)); 
}


fn greeting(greet: &str, name: &str){
    println!("{}, {} nice to meet you ", greet, name);
}

//Retunr. Dont use semicolon for fast return
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}