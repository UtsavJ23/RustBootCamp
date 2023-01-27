pub fn run(){
    println!("Hello from print.rs");
    println!("Using placeholders");
    println!("NUmber : {}", 1);
    println!("Learning {} for {}", "Rust", "Solana");
    println!("Learning {0} for {1} using {0} language for {1} {2}", "Rust", "Solana", "development");
    //Named traits
    println!("I am {name} and I want to become {aim}", name = "Utsav", aim = "Blockchain developer");
    //Placeholder traits
    println!("10 in Binary:{:b} Octal:{:o} Hexadecimal:{:x}", 10, 10, 10); 
    //Placeholder for debugging
    println!("{:?}", (12, true, "hello"));
    //Basic operations
    println!("10+10={}", 10+10);
}