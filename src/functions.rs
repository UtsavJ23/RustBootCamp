pub fn run(){
    println!("Hello world!");

    greetings("Utsav", 19);
    //bind function values to variables
    let sum = add(5,5);
    println!("Sum of function: {}", sum);

    //closure
    let z = 5;
    let sum_closure = | x: i32, y:i32 | x + y + z;
    println!("Sun of closure: {}", sum_closure(2,3));

}
pub fn greetings(name: &str, age: i32){
    println!("{} is {}", name, age);
}
pub fn add(x: i32, y: i32) -> i32 {
    x+y
}