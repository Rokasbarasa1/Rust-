
// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    //String type is mutable 
    let mut hello = String::from("hello");

    //GET LENGHT
    println!("Length: {}", hello.len());
    
    //Push characters
    hello.push('W');

    //Push string
    hello.push_str("orld!");

    println!("{}", hello);

    //Capacity in bytes
    println!("Capacity {}", hello.capacity());
    
    println!("Is empty: {}", hello.is_empty());

    println!("Contains 'world' {}", hello.contains("World"));

    //Replace 
    println!("Replace {}", hello.replace("World", "There"));

    //Loop trougt string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertion testing
    assert_eq!(3, s.len());
    assert_eq!(10, s.capacity());


    //println!("{}",assert_eq!(2, s.len()));

}