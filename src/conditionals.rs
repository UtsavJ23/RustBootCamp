pub fn run(){
    let age = 18;
    if age >= 21 {
    println!("Bartender: What would you like to drink");
    } else {
        println!("Sorry, you need to be 18"); 
    }

    //short hand if like ternary
    let is_of_age = if age>=21 {true} else {false};
    println!("Are you above 18? {}", is_of_age);
}