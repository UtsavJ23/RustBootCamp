use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let command = args[1].clone();
    let status = "100%";
     if(command == "hello"){
        println!("Hi, this application is running succesfully");
     }
     else if(command == "status"){
        println!("Status is {}", status);
     }
     else{
        println!("Its not a valid command");
     }
}