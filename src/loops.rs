pub fn run(){
    //infinite loop
    let mut count = 0;
    loop{
        count += 1;
        println!("{}", count);

        if count == 5 {
            break;
        }
    }
    count =0;
    while count <=50{
        let mut flag = true;
        if count %3 == 0 {
            print!("fizz");
            flag = false;
        }
        if count %5 == 0 {
            print!("buzz");
            flag = false;
        }
        if(flag){
            print!("{}", count);
        }
        count += 1;
        println!();
    }
    for mut x in 0..100{
        let mut flag = true;
        if x %3 == 0 {
            print!("fizz");
            flag = false;
        }
        if x %5 == 0 {
            print!("buzz");
            flag = false;
        }
        if(flag){
            print!("{}", x);
        }
        x += 1;
        println!();
    }
}