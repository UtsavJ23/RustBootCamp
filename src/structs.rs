use std::fmt::format;

//traditional struct
struct Color{
    r: u8,
    g: u8,
    b:u8
}
//tuple struct
struct color(u8, u8, u8);

struct Person{
    first: String,
    last: String
}
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person { 
            first: first.to_string(),
            last: last.to_string() }
    }
    //get full name using || self & format ||
    fn full_name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
    //set last name using || &mut self ||
    fn set_last(&mut self, last: &str){
        self.last= last.to_string();
    }
    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first, self.last)
    }
}
pub fn run(){
    let mut C = Color {
        r: 255,
        g:0,
        b:0
    };
    C.r = 250;//re-assignment
    println!("Color: {} {} {}", C.r, C.g, C.b);
    let mut c = color(255, 0, 0);
    c.0 = 250; //re-assignment
    println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("John", "Wick");
    println!("Person using new {} {}", p.first, p.last);
    println!("Person using format {}", p.full_name());
    p.set_last("Jacob");
    println!("Person using format {}", p.full_name());
    println!("Person tuple {:?}", p.to_tuple());
}