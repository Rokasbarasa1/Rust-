enum Movement{
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    //Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar Down up"),
        Movement::Left => println!("Avatar Left up"),
        Movement::Right => println!("Avatar Rigt up")
    }
}

pub fn run(){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}