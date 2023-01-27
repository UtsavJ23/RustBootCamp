pub fn run(){
    //Primitive array
    let arr1 = [1, 2, 3, 4];
    let arr2 = arr1;
    println!("Values of Arrays are: {:?}", (arr1, arr2));


    //with non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You need a reference (&) to point to the resource

    //Vector
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = &vec1;
    
    println!("Values of Vector are: {:?}", (&vec1, vec2));

}