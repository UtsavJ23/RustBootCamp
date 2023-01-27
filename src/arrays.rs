pub fn run(){
    let mut arr: [i32; 5] = [1,2,3,4,5];

    //re-assign value
    arr[2] = 20;
    println!("{:?}", arr);
    //array length
    println!("Array length is {}", arr.len());
    //array size in bytes
    println!("Array occupies {} bytes", std::mem::size_of_val(&arr));
    //array slices
    let slice: &[i32] = &arr[1..3];
    println!("sliced array {:?}", slice);

}