/*
Primitive types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays
*/

//Rust is statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and howe we use it

pub fn run(){
    let x=1;//default i32
    let y = 2.5;// default f64
    //adding explicit type
    let z: i64 = 32454246245325;
    //boolean
    let b= 10>20;
    let c = 'a';
    println!("Max i32:{}", std::i32::MAX);
    println!("Max i64:{}", std::i64::MAX);

    println!("{:?}", (x, y, z, b, c));
}