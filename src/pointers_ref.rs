// References Pointers - Pointers to a resource in memory
pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let  _arr2 = arr1; // arr2 is a pointer to arr1

    //non-primitive 

    //Vectors
    let vec1 = vec![1, 2, 3];   
    let vec2 = &vec1; // vec2 is a reference to vec1

    println!("Values: {:?}", (&vec1, vec2));
}