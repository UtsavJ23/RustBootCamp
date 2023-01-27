//typles group together values of different types
//tuples contain a MAX of 12 elements

pub fn run(){
    let tup: (&str, &str, i8) = ("Bradley", "Kolkata", 19);
    println!("{} is from {} and is {}", tup.0, tup.1, tup.2);
}