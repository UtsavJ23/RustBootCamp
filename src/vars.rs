//variables hold primitive data or references to data
//variables are immutable by default
//rust is a block-scoped language

pub fn run(){
    //unmutable variable
    let name = "Utsav";
    //mutable variable
    let mut age = 19;
    println!("My name is {} and I am {}", name, age);
    age = 20;
    println!("My name is {} and I am {}", name, age);

    //constant
    const ID: i32 = 01;
    println!("{}", ID);

    //assign multiple vars
    let (my_name, my_age) = ("Bhalu", "19");
    println!("{} is {}", my_name, my_age);
}