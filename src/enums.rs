enum movement { up, down, left, right }

fn avatar_movement(m: movement){
    match m{
        movement::up => println!("Avatar moving up"),
        movement::down => println!("Avatar moving down"),
        movement::left => println!("Avatar moving left"),
        movement::right => println!("Avatar moving right")
    }
}

pub fn run(){
    let avatar1= movement::up;
    let avatar2= movement::down;
    let avatar3= movement::left;
    let avatar4= movement::right;
    avatar_movement(avatar1);
    avatar_movement(avatar2);
    avatar_movement(avatar3);
    avatar_movement(avatar4);
}