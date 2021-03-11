

pub fn run(){
    let vec1 = vec! [1,2,3];
    //If you assign a non primitive to a a value the original one doesnt hold the value anymore
    let vec2 = &vec1;

    println!("Values : {:?}", (&vec1, vec2));
}