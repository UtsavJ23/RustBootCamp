pub fn run(){
    //immutable
    let immut_var = "hello";
    let immutvar = String::from("Hello ");
    //mutable
    let mut hello = String::from("Hello ");

    println!("Length: {}", hello.len());
    println!("{:?}", (immut_var, immutvar));
    println!("{}", hello);
    
    //push character
    hello.push('W');
    //push string
    hello.push_str("orld!");

    println!("{}", hello);

    //capacity in bytes
    println!("{}", hello.capacity());
    //is empty
    println!("{}", hello.is_empty());
    //contains
    println!("{}", hello.contains("World"));
    //replace
    println!("{}", hello.replace("World", "There"));
    //looping through whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }
    //creating a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion testing (just like solidity require)
    assert_eq!(2, s.len());

    println!("{}", s);

}