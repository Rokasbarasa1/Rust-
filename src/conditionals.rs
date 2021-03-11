pub fn run(){
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    //IF else 
    if age >= 21 && check_id || knows_person_of_age{
        println!("artender : what would you like to drink");
    } else if age < 21 && check_id{
        println!("artender : Sorry you have to leave",)
    } else {
        println!("artender : Il need to see your id",)
    }

    //Shorthand if
    let is_of_age = if age >= 21 { true} else {false};
    println!("Is of age {}", is_of_age);
}