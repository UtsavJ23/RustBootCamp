pub fn run(){
    let mut arr: Vec<i32> = vec![1,2,3,4,5];

    //re-assign value
    arr[2] = 20;
    //add in vector
    arr.push(23);
    arr.push(32);
    //pop from vector
    arr.pop();
    arr.pop();
    println!("{:?}", arr);
    //Vector length
    println!("Vector length is {}", arr.len());
    //Vector size in bytes
    println!("Vector occupies {} bytes", std::mem::size_of_val(&arr));
    //Vector slices
    let slice: &[i32] = &arr[1..3];
    println!("Sliced Vector {:?}", slice);
    //loop through vector
    for val in arr.iter(){
        print!("{} ", val);
    }
    //loop and mutate
    for val in arr.iter_mut(){
        *val *= 2;
    }
    println!("{:?} ", arr);
}